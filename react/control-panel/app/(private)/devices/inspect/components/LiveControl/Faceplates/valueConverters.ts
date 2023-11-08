import { ParameterId } from "@audiocloud/api"

export interface ConversionRules {
  'stringToString':   Record<ParameterId, Record<string, string>>,
  'stringToNumber':   Record<ParameterId, Record<string, number>>,
  'stringToBoolean':  Record<ParameterId, Record<string, boolean>>,
  'numberToString':   Record<ParameterId, Record<string, string>>,
  'numberToNumber':   Record<ParameterId, Record<string, number>>,
  'numberToBoolean':  Record<ParameterId, Record<string, boolean>>,
  'booleanToString':  Record<ParameterId, Record<string, string>>,
  'booleanToNumber':  Record<ParameterId, Record<string, number>>,
  'booleanToBoolean': Record<ParameterId, Record<string, boolean>>
}

export const convertToNumber = (value: string | number | boolean | null, parameter: ParameterId, customRules: ConversionRules): number => {
  if (typeof value === 'string') {
    const parameterRules = Object.entries(customRules.stringToNumber).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value)
      if (valueRules) return valueRules[1]
    }
    return parseFloat(value)
  }
  if (typeof value === 'number') {
    const parameterRules = Object.entries(customRules.numberToNumber).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value.toString())
      if (valueRules) return valueRules[1]
    }
    return value
  }
  if (typeof value === 'boolean') {
    const parameterRules = Object.entries(customRules.booleanToNumber).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value.toString())
      if (valueRules) return valueRules[1]
    }
    return value ? 1 : 0
  }
  throw Error(`${typeof value} value type in '${parameter}'`)
}

export const convertToString = (value: string | number | boolean | null, parameter: ParameterId, customRules: ConversionRules): string => {
  if (typeof value === 'string') {
    const parameterRules = Object.entries(customRules.stringToString).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value)
      if (valueRules) return valueRules[1]
    }
    return value
  }
  if (typeof value === 'number') {
    const parameterRules = Object.entries(customRules.numberToString).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value.toString())
      if (valueRules) return valueRules[1]
    }
    return value.toString()
  }
  if (typeof value === 'boolean') {
    const parameterRules = Object.entries(customRules.booleanToString).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value.toString())
      if (valueRules) return valueRules[1]
    }
    return value.toString()
  }
  throw Error(`${typeof value} value type in '${parameter}'`)
}

export const convertToBoolean = (value: string | number | boolean | null, parameter: ParameterId, customRules: ConversionRules): boolean => {
  if (typeof value === 'string') {
    const parameterRules = Object.entries(customRules.stringToBoolean).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value)
      if (valueRules) return valueRules[1]
    }
    return !!value
  }
  if (typeof value === 'number') {
    const parameterRules = Object.entries(customRules.numberToBoolean).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value.toString())
      if (valueRules) return valueRules[1]
    }
    return !!value
  }
  if (typeof value === 'boolean') {
    const parameterRules = Object.entries(customRules.booleanToBoolean).find(item => item[0] === parameter)
    if (parameterRules) {
      const valueRules = Object.entries(parameterRules[1]).find(item => item[0] === value.toString())
      if (valueRules) return valueRules[1]
    }
    return value
  }
  throw Error(`${typeof value} value type in '${parameter}'`)
}