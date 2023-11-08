import { DeviceReportsType } from "@/types"

export const defaultReports: Record<string, DeviceReportsType> = {
  'btrmkr_ml_1': {
    clip:                   [60, 60],
    gainReduction:          [40, 40],
    peakLevel:              [100, 100],
    rmsLevel:               [80, 80]
  },
  'distopik_1176_1': {
    gainReduction:          [40, 40],
    volume:                 [40, 40]
  },
  'distopik_adu_1': {
    gainReduction:          [0, 0]
  },
  'distopik_bwl_1': {
    gainReduction:          [0, 0]
  },
  'distopik_eqp1_1': {
    //
  },
  'distopik_fc670_1': {
    inputLevel:             [60, 60],
    gainReduction:          [40, 40],
    outputLevel:            [100, 100]
  },
  'distopik_la2a_1': {
    gainReduction:          [0, 0],
    vuLevel:                [0, 0]
  },
  'distopik_meq_1': {
    //
  },
  'distopik_vca_1': {
    gainReduction:          [0, 0]
  },
  'elysia_karacter_1': {
    //
  },
  'elysia_museq_1': {
    //
  },
  'elysia_nvelope_1': {
    //
  },
  'elysia_xfilter_1': {
    //
  },
  'elysia_xpressor_1': {
    gainReduction:          [0, 0]
  },
  'gyraf_g24_1': {
    gainReduction:          [0, 0]
  },
  'tierra_gravity_1': {
    gainReduction:          [40, 40]
  },
  'neve_1084_1': {
    clip:                   [0, 0]
  }
}