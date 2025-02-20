#!/usr/bin/env python3

# This script could be used for Actix Web multipart example test
# just start server and run client.py

import json
import asyncio
import aiohttp


async def req():
    resp = await aiohttp.ClientSession().request(
        "post",
        "http://localhost:5000/index",
        data=json.dumps({"name": "Test user", "number": 100}),
        headers={"content-type": "application/json"},
    )
    print(str(resp))
    print(await resp.text())
    # assert 200 == resp.status


asyncio.get_event_loop().run_until_complete(req())
