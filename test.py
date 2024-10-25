import batch_http_request
import asyncio

reqs = []

for i in range(10):
    reqs.append(batch_http_request.Request(f"https://google.com", "GET"))

async def main():
    response = await batch_http_request.batch_request(reqs)
    print([bytes(r.body).decode('utf-8') for r in response])
    
if __name__ == '__main__':
    asyncio.run(main())