import discord
from roll_parser import compute
from os import getenv


class RollTheDiceClient(discord.Client):
    async def on_ready(self):
        print('Logged on as {0}!'.format(self.user))

    async def on_message(self, message):
        print('Message from {0.author}: {0.content}'.format(message))
        if message.author.name != self.user.name and message.content[:5] == "/rtd ":
            res = compute(message.content.split("/rtd ")[1])
            await message.channel.send(res)


if __name__ == "__main__":
    client = RollTheDiceClient()
    token = getenv("DISCORD_TOKEN")
    if token is None:
        print("DISCORD_TOKEN environment variable is not set")
        exit()
    client.run(token)
