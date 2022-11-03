#!/usr/bin/env node
/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

import {compile} from 'json-schema-to-typescript'
import * as fs from "fs/promises";
import {format as prettier} from "prettier"
import minimist from "minimist"
import {snakeCase} from "change-case"
import yaml from 'yaml'
import path from 'path';

async function main(): Promise<void> {
    let args = minimist(process.argv.slice(2))
    const {output = './src'} = args
    const inputFileNames = args['_']

    if (!inputFileNames.length) {
        throw new Error('No input files specified')
    }

    await fs.mkdir(output, {recursive: true}).catch(() => {
    })

    await fs.writeFile(`${output}/base.ts`, Buffer.from('ZXhwb3J0IHR5cGUgUmVxdWVzdDxCPiA9CiAgICB8IHsgbWV0aG9kOiAnZ2V0JywgaGVhZGVycz86IFJlY29yZDxzdHJpbmcsIGFueT4sIHBhdGg6IHN0cmluZyB9CiAgICB8IHsgbWV0aG9kOiAncG9zdCcgfCAnZGVsZXRlJyB8ICdwYXRjaCcgfCAncHV0JywgYm9keT86IEIsIGhlYWRlcnM/OiBSZWNvcmQ8c3RyaW5nLCBhbnk+LCBwYXRoOiBzdHJpbmcgfQoKZXhwb3J0IGludGVyZmFjZSBSZXF1ZXN0ZXIgewogICAgcmVxdWVzdDxCLCBULCBFPihyZXF1ZXN0OiBSZXF1ZXN0PEI+KTogUHJvbWlzZTxSZXN1bHQ8VCwgRT4+Owp9CgpleHBvcnQgdHlwZSBSZXN1bHQ8VCwgRT4gPQogICAgfCB7IG9rOiBULCBlcnJvcjogbnVsbCwgaXNfb2s6IHRydWUsIGlzX2Vycm9yOiBmYWxzZSB9CiAgICB8IHsgb2s6IG51bGwsIGVycm9yOiBFLCBpc19vazogZmFsc2UsIGlzX2Vycm9yOiB0cnVlIH0K', 'base64'))

    for (const inputFileName of inputFileNames) {
        await generate(inputFileName, `${output}/${path.parse(inputFileName).name}.ts`)
    }
}

async function generate(fileName: string, outputFileName: string) {
    let contents = (await fs.readFile(fileName)).toString('utf-8')
    let json

    if (fileName.endsWith('.json')) {
        json = JSON.parse(contents);
    } else {
        json = yaml.parse(contents)
    }
    json.definitions = json.components.schemas;

    for (const d in json.definitions) {
        delete json.definitions[d].title
    }

    let buf = `import {Requester, Result} from './base'\n`
    buf += `export * from './base'\n`

    buf += await compile(json, 'CloudApi', {additionalProperties: false, unreachableDefinitions: true, format: false})

    buf += 'export class Client {\n'
    buf += '  constructor(private readonly requester: Requester) {}\n'

    for (let path in json.paths) {
        let pathItem = json.paths[path];
        for (let method in pathItem) {
            let operation = pathItem[method];
            if (operation.operationId) {
                const {operationId} = operation;
                const params = operation.parameters?.filter((p: any) => p.in === 'path' || p.in == 'header').map((p: any) => `${snakeCase(p.name)}: ${getTypeName(p)}`) || []
                const bodyArg = operation.requestBody?.content['application/json'] ? getTypeName(operation.requestBody?.content['application/json']) : null

                let responseType = 'void'
                let errorType = 'Error'

                if (operation.responses['200']?.content['application/json']) {
                    responseType = getTypeName(operation.responses['200']?.content['application/json'])
                }

                for (let code in operation.responses) {
                    if (code.startsWith('4') || code.startsWith('5')) {
                        if (operation.responses[code].content['application/json']) {
                            errorType = getTypeName(operation.responses[code].content['application/json'])
                            break;
                        }
                    }
                }


                const requestObj: Record<string, string> = {
                    path: `\`${path.replace(/\/{/g, '/${')}\``,
                    method: `"${method}"`
                }

                let comments = []

                if (operation.description) {
                    comments.push(...operation.description.split('\n').map((l: string) => l.trim()))
                }

                for (let param of operation.parameters || []) {
                    if (param.description) {
                        comments.push(`@param ${snakeCase(param.name)} ${param.description}`.trim())
                    }
                }

                if (bodyArg) {
                    requestObj.body = 'body'
                    params.push(`body: ${bodyArg}`)
                    comments.push(`@param body ${operation.requestBody?.description || 'Request body'}`.trim())
                }

                if (comments.length > 0) {
                    buf += `  /**\n`
                    for (let comment of comments) {
                        buf += `   * ${comment}\n`
                    }
                    buf += `   */\n`
                }

                requestObj.headers = '{'
                for (let param of operation.parameters || []) {
                    if (param.in === 'header') {
                        requestObj.headers += `'${param.name}': ${snakeCase(param.name)},`
                    }
                }
                requestObj.headers += '}'

                let requestObjStr = '{'
                for (let key in requestObj) {
                    requestObjStr += `${key}: ${requestObj[key]},`
                }
                requestObjStr += '}'

                buf += `  async ${operationId}(${params.join(', ')}): Promise<Result<${responseType}, ${errorType}>> {\n`
                buf += `    return this.requester.request(${requestObjStr});\n`
                buf += `  }\n`
            }
        }
    }

    buf += '}\n'

    await fs.writeFile(outputFileName, Buffer.from(prettier(buf, {
        semi: false,
        parser: 'babel-ts',
        printWidth: 120
    }), 'utf-8'))
}

function getTypeName(p: any) {
    if (p.schema && p.schema.$ref) {
        return p.schema.$ref.substring(p.schema.$ref.lastIndexOf('/') + 1)
    } else if (p.schema.type) {
        switch (p.schema.type) {
            case 'string':
                return 'string'
            case 'integer':
            case 'number':
                return 'number'
            default:
                return 'any'
        }
    } else {
        return 'any'
    }
}

main().catch(console.error)
