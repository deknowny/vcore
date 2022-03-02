import vcore_ext


cl = vcore_ext.collector.HandlersCollector()


@cl.add
async def handle_event(ctx: NewEvent):
    pass


async def main():
    pass