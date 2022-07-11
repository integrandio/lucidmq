from sanic import Sanic, response
from sanic.log import logger
from sanic.response import text
from sanic.response import json as sanicJson
import pylucidmq

import json 

from collections import defaultdict
from typing import Dict, FrozenSet, Iterable
from sanic.router import Route


app = Sanic("Streamcel")
lucidmq = pylucidmq.LucidMQ("../test_log", 1000, 500)
consumer = lucidmq.new_consumer("topic1", "group2")
producer = lucidmq.new_producer("topic1")
app.ctx.consumer = consumer
app.ctx.producer = producer

@app.get("/")
async def hello_world(request):
    return text("Welcome to Streamcel app")

@app.get("/<topic>")
async def getTopicInfo(request, topic):
    oldest_offset = app.ctx.consumer.get_oldest_offset()
    latest_offset = app.ctx.consumer.get_latest_offset()
    data_dict = {
        "topic_name": "topic1",
        "oldest_offset": oldest_offset,
        "latest_offset": latest_offset
    }
    return sanicJson(data_dict)


# @app.post("/produce/<topic>")
# async def produce(request, topic):
#     key = "key{0}".format(x).encode()
#     value = "value{0}".format(x).encode()
#     x = pylucidmq.Message(key, value)
#     producer.produce_message(x)

@app.get("/consume/<topic>/<offset:int>/<total:int>")
async def consumer(request, topic, offset, total):
    print("consuming....")
    messages = app.ctx.consumer.fetch(offset, total)
    messageList = []
    for message in messages:
        key = bytes(message.key)
        value = bytes(message.value)
        dictKay = key.decode("utf-8")
        dictValue = value.decode("utf-8")
        #dictMessage = {dictKay: dictValue}
        res = json.loads(dictValue)
        print(res)
        messageList.append(res)
    print(messageList)
    return sanicJson({"data": messageList})

@app.websocket("/feed")
async def feed(request, ws):
    print("inside feed function")
    data = await ws.recv()
    print("Received: " + data)
    checks = 0
    while checks < 10:
        messages = app.ctx.consumer.poll(1000)
        for message in messages:
            key = bytes(message.key)
            value = bytes(message.value)
            dictKay = key.decode("utf-8")
            dictValue = value.decode("utf-8")
            res = json.loads(dictValue)
            #dictMessage = {dictKay: dictValue}
            json_object = json.dumps(res, indent = 1) 
            print(json_object)
            await ws.send(json_object)
            wsRes = await ws.recv()
            print("Received: " + wsRes)
        print("Checking for something")
        checks += 1

# CORS BS
def _compile_routes_needing_options(
    routes: Dict[str, Route]
) -> Dict[str, FrozenSet]:
    needs_options = defaultdict(list)
    # This is 21.12 and later. You will need to change this for older versions.
    for route in routes.values():
        if "OPTIONS" not in route.methods:
            needs_options[route.uri].extend(route.methods)

    return {
        uri: frozenset(methods) for uri, methods in dict(needs_options).items()
    }


def _options_wrapper(handler, methods):
    def wrapped_handler(request, *args, **kwargs):
        nonlocal methods
        return handler(request, methods)

    return wrapped_handler


async def options_handler(request, methods) -> response.HTTPResponse:
    resp = response.empty()
    _add_cors_headers(resp, methods)
    return resp


def setup_options(app: Sanic, _):
    app.router.reset()
    needs_options = _compile_routes_needing_options(app.router.routes_all)
    for uri, methods in needs_options.items():
        app.add_route(
            _options_wrapper(options_handler, methods),
            uri,
            methods=["OPTIONS"],
        )
    app.router.finalize()

def _add_cors_headers(response, methods: Iterable[str]) -> None:
    allow_methods = list(set(methods))
    if "OPTIONS" not in allow_methods:
        allow_methods.append("OPTIONS")
    headers = {
        "Access-Control-Allow-Methods": ",".join(allow_methods),
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Credentials": "true",
        "Access-Control-Allow-Headers": (
            "origin, content-type, accept, "
            "authorization, x-xsrf-token, x-request-id"
        ),
    }
    response.headers.extend(headers)


def add_cors_headers(request, response):
    print("Checking request headers...")
    if request.method != "OPTIONS":
        if request.route is None:
            print(request)
        else:
            methods = [method for method in request.route.methods]
            _add_cors_headers(response, methods)

# Add OPTIONS handlers to any route that is missing it
app.register_listener(setup_options, "before_server_start")

# Fill in CORS headers
app.register_middleware(add_cors_headers, "response")