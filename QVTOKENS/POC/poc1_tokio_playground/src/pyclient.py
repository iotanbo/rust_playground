
"""


"""

import asyncio


RX_TIMEOUT_SEC = 5


async def tcp_echo_client(message):
    try:
        # '192.168.1.200' '127.0.0.1'  '192.168.1.201'
        reader, writer = await asyncio.open_connection('192.168.1.201', 9556)
        # print('* CONNECTED, sending: %r' % message)
        writer.write(message)
        try:
            _data = await asyncio.wait_for(reader.read(128), timeout=RX_TIMEOUT_SEC)
            if not _data:
                # print('  -> conn closed by remote peer')
                writer.close()
                return
            # print('Received: %r' % _data.decode())
        except asyncio.TimeoutError:
            # print('Rx timeout')
            writer.close()
            return
        except ConnectionResetError:
            # print('  -> conn was reset by remote peer')
            writer.close()
            return

        # print('Closed the socket')
        writer.close()

    except Exception as e:
        print(f'  -> REFUSED to connect: [{e.__class__.__name__}] {str(e)}')


async def run_tcp_clients(data_list, total_clients: int):

    # loop = asyncio.get_event_loop()
    all_tasks = []
    for i in range(total_clients):
        all_tasks.append(asyncio.create_task(tcp_echo_client(data_list[i])))

    _done, _pending = await asyncio.wait(all_tasks, timeout=RX_TIMEOUT_SEC,
                                         return_when=asyncio.ALL_COMPLETED)


if __name__ == '__main__':

    data = []
    # data.append(bytes(b"\xFF") * 16)


    for token in range(1000):
        data.append(token.to_bytes(16, byteorder='little'))

    data.append(0xFFFFFF.to_bytes(16, byteorder='little'))

    for _ in range(20):
        asyncio.run(run_tcp_clients(data, 1000))

    data[0] = bytes(b"\xFF") * 16

    # Extra message to quit the server
    asyncio.run(run_tcp_clients(data, 1))

