#!/bin/bash
keyfilename=$1
keyfilename=${keyfilename:-app.key}
crtfilename=$2
crtfilename=${crtfilename:-app.crt}
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout ${keyfilename} -out ${crtfilename}
