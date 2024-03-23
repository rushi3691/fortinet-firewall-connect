import requests as req
import bs4 as bs
import argparse

FORTI_URL = "http://10.250.209.251:1000/login?05"
HEADERS = {'User-Agent': 'Mozilla/5.0', 'Connection': 'keep-alive'}


def extract_magic():
    resp = req.get(FORTI_URL, headers=HEADERS)
    soup = bs.BeautifulSoup(resp.text, 'html.parser')
    magic = soup.find('input', {'name': 'magic'})['value']
    return magic


def login(username: str, password: str):
    magic = extract_magic()
    print("magic", magic)
    data = {
        '4Tredir': FORTI_URL,
        'magic': magic,
        'username': username,
        'password': password
    }
    resp = req.post(FORTI_URL, data=data, headers=HEADERS)

    keep_alive_url = bs.BeautifulSoup(
        resp.text, 'html.parser'
    ).find('script').text.split('"')[1]
    print(keep_alive_url)

    session_id = keep_alive_url.split('?')[1]
    print(session_id)


parser = argparse.ArgumentParser()
parser.add_argument("--username", help="username", required=True)
parser.add_argument("--password", help="password", required=True)
args = parser.parse_args()
username = args.username
password = args.password

login(username, password)
