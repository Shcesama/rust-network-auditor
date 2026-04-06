# Vulnerable Target Simulator
# This script will simulate a system with open ports and vulnerabilities for the auditor to find.

import socket
import threading
import time

def ftp_server():
    # Simulates an FTP server on port 21
    # Very basic: just sends a banner and handles anonymous login attempt
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', 21))
    s.listen(5)
    print("[*] Target: Simulating FTP on port 21")
    while True:
        client, addr = s.accept()
        client.send(b"220 (vsFTPd 3.0.3)\r\n")
        data = client.recv(1024)
        if b"USER anonymous" in data:
            client.send(b"331 Please specify the password.\r\n")
            data = client.recv(1024)
            client.send(b"230 Login successful.\r\n")
        client.close()

def http_server():
    # Simulates an HTTP server on port 80
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', 80))
    s.listen(5)
    print("[*] Target: Simulating HTTP on port 80")
    while True:
        client, addr = s.accept()
        data = client.recv(1024)
        if b"HEAD /" in data:
            client.send(b"HTTP/1.0 200 OK\r\nServer: Apache/2.4.41 (Ubuntu)\r\n\r\n")
        client.close()

def https_server():
    # Simulates an HTTPS-like server on port 443 (just TCP)
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', 443))
    s.listen(5)
    print("[*] Target: Simulating HTTPS on port 443")
    while True:
        client, addr = s.accept()
        # Just close or send dummy banner
        client.close()

if __name__ == "__main__":
    t1 = threading.Thread(target=ftp_server, daemon=True)
    t2 = threading.Thread(target=http_server, daemon=True)
    t3 = threading.Thread(target=https_server, daemon=True)
    
    t1.start()
    t2.start()
    t3.start()
    
    print("[!] Target Environment Ready. Press Ctrl+C to stop.")
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("[*] Shutting down target.")
