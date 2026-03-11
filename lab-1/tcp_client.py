import socket

def tcp_client():
    server_ip = '127.0.0.1'
    server_port = 3939

    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
    try:
        client.connect((server_ip, server_port))
        print(f"已连接到服务器 {server_ip}:{server_port}")
        print("第一行输入姓名、学号和班级，至少输入5行内容。\n")

        line_count = 0
        while True:
            message = input(f"第{line_count + 1}行> ")
            if message.lower() == 'exit':
                print("断开连接。")
                break

            client.send(message.encode('utf-8'))
            line_count += 1

            response = client.recv(4096)
            print(f"服务器回复: {response.decode('utf-8')}")

            if line_count >= 5:
                cont = input("已输入5行，是否继续？(y/n): ")
                if cont.lower() != 'y':
                    break

    except ConnectionRefusedError:
        print(f"连接被拒绝，请检查服务器 {server_ip}:{server_port} 是否在线。")
    except Exception as e:
        print(f"发生错误: {e}")
    finally:
        client.close()
        print("连接已关闭。")

if __name__ == '__main__':
    tcp_client()
