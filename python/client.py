# HTTP Requests client library
import requests
# Fake data
from faker import Faker

# Fake data generator
fake_gen = Faker('en_US')

# Quick test
# for _ in range(1000):
#     print(fake_gen.name())
# response = requests.get('http://127.0.0.1:8083/ping')
# print(response.status_code)
# print(response.text)

response = requests.get('http://127.0.0.1:8083/customers')
print(response.status_code)
print(response.text)