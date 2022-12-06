import { readFileSync } from 'fs'
import { resolve } from 'path'
import path from 'path'
import { fileURLToPath } from 'url'
import { performance } from 'perf_hooks'

import NodeRSA from 'node-rsa'

const key = new NodeRSA({b: 512})

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const fileName = process.argv[3]

const file = readFileSync(resolve(__dirname, '..', '..', 'dataset', fileName + '.txt'), 'utf8')

const data = Buffer.from(file)



// ---------------- Time to encrypt --------------- 
let startTime = performance.now()
const encrypted = key.encrypt(data, 'base64')
let endTime = performance.now()
// console.log('encrypted: ', encrypted)
console.log(`encrypt ${fileName} ${endTime - startTime} milliseconds`)
// ---------------- Time to encrypt --------------- 

// console.log({ encrypted: encrypted.toString() })

// ---------------- Time to decrypt --------------- 
startTime = performance.now()
const decrypted = key.decrypt(encrypted, 'utf8')
endTime = performance.now()
console.log(`decrypt ${fileName} ${endTime - startTime} milliseconds`)
// ---------------- Time to decrypt --------------- 

// console.log(decrypted)