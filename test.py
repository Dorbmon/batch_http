import batch_http
import asyncio

reqs = []

for i in range(10):
    reqs.append(batch_http.Request(f"https://api-test.rumibot.com/ping", "GET"))

async def main():
    response = await batch_http.batch_request(reqs)
    print([bytes(r.body).decode('utf-8') for r in response])
    
if __name__ == '__main__':
    asyncio.run(main())