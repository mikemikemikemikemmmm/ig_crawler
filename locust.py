from locust import HttpUser, task, between
import random


class WebsiteUser(HttpUser):
    wait_time = between(0, 5)
    user_list = [
        "tsai_ingwen",
        "ladyflavor",
        "rayduenglish",
        "zamy_ding",
        "helloiamhook",
    ]

    @task
    def index(self):
        random_element = random.choice(self.user_list)
        self.client.get("/" + random_element)
