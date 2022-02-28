import os
import asyncio

import vcore_ext


async def main():
    api = vcore_ext.api.APIExecutor(token=os.getenv("USER_TOKEN"))
    response = await api.call("users.get", user_ids=None)
    print(response.get(0, "first_name"))


asyncio.run(main())
