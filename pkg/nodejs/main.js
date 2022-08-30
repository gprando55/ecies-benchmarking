import { encrypt, decrypt, PrivateKey } from 'eciesjs'
import { readFileSync } from 'fs'
import { resolve } from 'path'
import path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const fileName = process.argv[3]

console.log({ fileName })

const file = readFileSync(resolve(__dirname, '..', '..', 'dataset', fileName + '.txt'), 'utf8')

console.log(file)

const k1 = new PrivateKey()
const data = Buffer.from(file)

const encrypted = encrypt(k1.publicKey.toHex(), data)

console.log({ encrypted: encrypted.toString() })

const decrypted = decrypt(k1.toHex(), encrypted).toString()
console.log(decrypted)