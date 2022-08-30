import sys
from ecies.utils import generate_eth_key, generate_key
from ecies import encrypt, decrypt

def getFileName():
  n = len(sys.argv)
  if n<3:
    raise Exception("FileName is required, pls running with flag `python3 main.py --file filename`")
  print("Total arguments passed:", n, sys.argv)
  return sys.argv[2]

def readFile(fileName):
  print('fileName -> ' + fileName)
  in_file = open("../../dataset/"+fileName+".txt", "rb") # opening for [r]eading as [b]inary
  data = in_file.read() 
  return data

def encryptFile(pk_bytes, file):
  return encrypt(pk_bytes, file)

def decryptFile(sk_bytes, fileEncrypted):
  return decrypt(sk_bytes, fileEncrypted)

def generateEthKey():
  eth_k = generate_eth_key()
  sk_hex = eth_k.to_hex()  # hex string
  pk_hex = eth_k.public_key.to_hex()  # hex string
  print('key eth generated')
  return sk_hex, pk_hex

def generateKey():
  secp_k = generate_key()
  sk_bytes = secp_k.secret  # bytes
  pk_bytes = secp_k.public_key.format(True)  # bytes
  print('key eth generated')
  return sk_bytes, pk_bytes


def main():
  fileName = getFileName()
  
  file = readFile(fileName)
  
  sk_bytes, pk_bytes = generateKey()

  fileEncrypted = encryptFile(pk_bytes, file)
  fileDecrypted = decryptFile(sk_bytes, fileEncrypted)

  print(fileDecrypted)

main()