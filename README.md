# @ahau2019/decode v1.0.3
![image](https://img.shields.io/npm/v/@ahau2019/decode)
![image](https://img.shields.io/github/license/saltires/saltire-decode)

This library provides a fast solution for decrypting data. It is based on WASM

## Installation

Using npm:
```shell
$ npm i --save @ahau2019/decode
```

In Webpack:
```js
const decode = import("@ahau2019/decode");

// The data you need to decode
const data = ''

decode.then(decode => {
  const realData = decode.uncode(data)
}).catch(err => console.log(err));
```

## API
```js
/**
 *  还原真实数据
 * @param {*} 后台加密后的数据（短格式）
 * @return String
 */
function uncode(data: String): String {

}

/**
 *  还原真实数据
 * @param {*} 后台加密后的数据（长格式）
 * @return String
 */
function uncode_long(data: String): String {

}
```