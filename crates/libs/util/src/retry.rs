// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::{
    ErrorCode,
    runtime::executor::{BoxedCancelToken, Timer},
};
use std::{pin::Pin, time::Duration};

/// TimeCounter is used to track elapsed time and remaining time for operations.
struct TimeCounter {
    timeout: Duration,
    start: std::time::Instant,
}

impl TimeCounter {
    pub fn new(timeout: Duration) -> Self {
        TimeCounter {
            timeout,
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn remaining(&self) -> mssf_core::Result<Duration> {
        if self.elapsed() < self.timeout {
            Ok(self.timeout - self.elapsed())
        } else {
            Err(ErrorCode::FABRIC_E_TIMEOUT.into())
        }
    }

    /// returns a future that will sleep until the remaining time is up.
    pub fn sleep_until_remaining(
        &self,
        timer: &dyn Timer,
    ) -> mssf_core::Result<impl Future<Output = ()>> {
        let remaining = self.remaining()?;
        Ok(timer.sleep(remaining))
    }
}

#[derive(Default)]
pub struct OperationRetryerBuilder {
    timer: Option<Box<dyn Timer>>,
    default_timeout: Option<Duration>,
    max_retry_interval: Option<Duration>,
}

impl OperationRetryerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// With a runtime timer to use for sleeping.
    pub fn with_timer(mut self, timer: Box<dyn Timer>) -> Self {
        self.timer = Some(timer);
        self
    }

    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = Some(timeout);
        self
    }

    pub fn with_max_retry_interval(mut self, interval: Duration) -> Self {
        self.max_retry_interval = Some(interval);
        self
    }

    pub fn build(self) -> OperationRetryer {
        OperationRetryer::new(
            self.timer.unwrap_or(Box::new(crate::tokio::TokioTimer)),
            self.default_timeout.unwrap_or(Duration::from_secs(30)),
            self.max_retry_interval.unwrap_or(Duration::from_secs(5)),
        )
    }
}

/// A helper to retry an operation with transient error and timeout.
pub struct OperationRetryer {
    timer: Box<dyn Timer>,
    default_timeout: Duration,
    max_retry_interval: Duration,
}

impl OperationRetryer {
    pub fn builder() -> OperationRetryerBuilder {
        OperationRetryerBuilder::new()
    }

    fn new(timer: Box<dyn Timer>, default_timeout: Duration, max_retry_interval: Duration) -> Self {
        OperationRetryer {
            timer,
            default_timeout,
            max_retry_interval,
        }
    }

    /// Run the operation with retry on transient errors and timeouts.
    /// User can provide a total timeout and a cancel token.
    pub async fn run<T, F, Fut>(
        &self,
        op: F,
        timeout: Option<Duration>,
        token: Option<BoxedCancelToken>,
    ) -> mssf_core::Result<T>
    where
        F: Fn(Duration, Option<BoxedCancelToken>) -> Fut,
        Fut: Future<Output = mssf_core::Result<T>> + Send,
        T: Send,
    {
        let timeout = timeout.unwrap_or(self.default_timeout);
        let timer = TimeCounter::new(timeout);
        let mut cancel: Pin<Box<dyn std::future::Future<Output = ()> + Send>> =
            if let Some(t) = &token {
                t.wait()
            } else {
                Box::pin(std::future::pending())
            };
        loop {
            let res = tokio::select! {
                _ = timer.sleep_until_remaining(self.timer.as_ref())? => {
                    // Timeout reached, return error.
                    return Err(ErrorCode::FABRIC_E_TIMEOUT.into());
                }
                _ = &mut cancel => {
                    // Cancellation requested, return error.
                    return Err(ErrorCode::E_ABORT.into());
                }
                // Run the operation with the remaining time and cancel token.
                res = op(timer.remaining()?, token.clone()) => res,
            };
            match res {
                Ok(r) => return Ok(r),
                Err(e) => match e.try_as_fabric_error_code() {
                    Ok(ec) => {
                        if ec == ErrorCode::FABRIC_E_TIMEOUT || ec.is_transient() {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "Operation transient error {ec}. Remaining time {:?}. Retrying...",
                                timer.remaining()?
                            );
                            // do nothing, retry.
                        } else {
                            return Err(e);
                        }
                    }
                    _ => return Err(e),
                },
            }
            // sleep for a while before retrying.
            tokio::select! {
                _ = self.timer.sleep(self.max_retry_interval) => {},
                _ = timer.sleep_until_remaining(self.timer.as_ref())? => {
                    // Timeout reached, return error.
                    return Err(ErrorCode::FABRIC_E_TIMEOUT.into());
                }
                _ = &mut cancel => {
                    // Cancellation requested, return error.
                    return Err(ErrorCode::E_ABORT.into());
                }
            }
        }
    }
}
