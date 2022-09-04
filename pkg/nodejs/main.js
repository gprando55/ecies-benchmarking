import { encrypt, decrypt, PrivateKey } from 'eciesjs'
import { readFileSync } from 'fs'
import { resolve } from 'path'
import path from 'path'
import { fileURLToPath } from 'url'
import { performance } from 'perf_hooks'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const fileName = process.argv[3]

// console.log({ fileName })

const file = readFileSync(resolve(__dirname, '..', '..', 'dataset', fileName + '.txt'), 'utf8')

// console.log(file)

const k1 = new PrivateKey()
const data = Buffer.from(file)


// ---------------- Time to encrypt --------------- 
let startTime = performance.now()
const encrypted = encrypt(k1.publicKey.toHex(), data)
let endTime = performance.now()
console.log(`encrypt ${fileName} ${endTime - startTime} milliseconds`)
// ---------------- Time to encrypt --------------- 

// console.log({ encrypted: encrypted.toString() })

// ---------------- Time to decrypt --------------- 
startTime = performance.now()
const decrypted = decrypt(k1.toHex(), encrypted).toString()
endTime = performance.now()
console.log(`decrypt ${fileName} ${endTime - startTime} milliseconds`)
// ---------------- Time to decrypt --------------- 

// console.log(decrypted)