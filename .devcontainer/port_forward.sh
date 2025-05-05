#!/bin/bash

socat TCP-LISTEN:19080,fork TCP:onebox:19080 & 
socat TCP-LISTEN:19000,fork TCP:onebox:19000