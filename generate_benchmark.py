# -*- coding: utf-8 -*-

import sys
import json
import pygsheets
from google.oauth2 import service_account
# from google.oauth2.credentials import Credentials

args = sys.argv


service_info = {
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
}

service_info['client_email'] = args[1]
with open(args[2], encoding="utf-8") as f:
    service_info['private_key'] = f.read()

SCOPES = (
    'https://www.googleapis.com/auth/spreadsheets',
    'https://www.googleapis.com/auth/drive'
)

my_credentials = service_account.Credentials.from_service_account_info(service_info, scopes=SCOPES)

gc = pygsheets.authorize(custom_credentials=my_credentials)

# need create file on googlesheets and share with this servive account
sh = gc.open("benchmark")

# select sheet1
worksheet = sh.sheet1

with open('result.json', encoding='utf-8') as f:
    line = f.read()
    data = json.loads(line)

tdata = [[str(t) for t in data['results'][0]['times']]]

# update data
worksheet.update_values('1:1', tdata)

# create file
# sh = gc.create("ssss")

# sh.share('betta0801@gmail.com', perm_type='user', role='writer')
