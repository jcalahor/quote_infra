import pytest
import subprocess
import time
import os
from kafka import KafkaProducer
from dotenv import load_dotenv
import json
import requests
from datetime import datetime


load_dotenv()


@pytest.fixture(scope="session", autouse=True)
def start_services():
    try:
        # Start the services
        subprocess.run(["./tests/integration/start_quote_services.sh"], check=True)

        time.sleep(10)

        yield  # Run tests

    finally:
        time.sleep(3)
        subprocess.run(["./tests/integration/stop_quote_services.sh"], check=True)


def validate_by_rate(producer, topic, formatted_date, rate1, rate2):
    quotes = [
        {
            "date": formatted_date,
            "rate": rate1,
            "quote": "USD",
            "base": "ABC",
            "timestamp": 1739318341,
        },
        {
            "date": formatted_date,
            "rate": rate2,
            "quote": "USD",
            "base": "XYZ",
            "timestamp": 1739318341,
        },
    ]

    for q in quotes:
        msg = json.dumps(q).encode("utf-8")
        producer.send(topic, msg)

    payload = {"quote": "ABC", "base": "XYZ"}

    url = "http://localhost:4000/quote"
    time.sleep(2)
    response = requests.get(url, json=payload)
    assert (
        response.status_code == 200
    ), f"Unexpected status code: {response.status_code}"

    resp = response.json()
    assert resp["quote"] == "ABC"
    assert resp["base"] == "XYZ"
    assert resp["rate"] == float(rate1 / rate2)


def test_quote():
    KAFKA_BROKER = os.getenv("KAFKA_BROKER", "localhost:19092")
    TOPIC = os.getenv("KAFKA_TOPIC", "quote_feed")
    producer = KafkaProducer(bootstrap_servers=KAFKA_BROKER)
    current_datetime = datetime.now()
    formatted_date = current_datetime.strftime("%Y-%m-%d")

    validate_by_rate(producer, TOPIC, formatted_date, 2.0, 0.5)
    validate_by_rate(producer, TOPIC, formatted_date, 10.0, 5.0)
