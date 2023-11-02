import { InstanceParametersConfig } from "@/types"

export const defaultStates: Record<string, InstanceParametersConfig> = {
  'btrmkr_ml_1': {
    channel_ids: ['0'],
    parameters: {
      inputLevel:                 [0],
      outputLevel:                [0],
      measureOutput:              [false],
      attack:                     [15],
      release:                    [600],
      clippiterAmount:            [50],
      clippiterEnabled:           [false],
      intelligentReleaseEnabled:  [false],
      midSideEnabled:             [false],
      midSideWidth:               [0],
      evenColorAmount:            [10],
      evenColorDrive:             [30],
      evenColorEnabled:           [false],
      evenColorFrequency:         [56],
      oddColorAmount:             [55],
      oddColorDrive:              [45],
      oddColorEnabled:            [false],
      oddColorFrequency:          [4000],
      processingEnabled:          [true]
    },
    wet: 100
  },
  'distopik_1176_1': {
    channel_ids: ['0'],
    parameters: {
      inputLevel:                 [-24, -24],
      outputLevel:                [-24, -24],
      attack:                     [4, 4],
      release:                    [4, 4],
      ratio:                      [4, 4],
      amplifierMode:              ['a'],
      link:                  [true]
    },
    wet: 100
  },
  'distopik_adu_1': {
    channel_ids: ['0', '1'],
    parameters: {
      drive:                      [6, 6],
      distortionType:             ['TR', 'TR'],
      outputLevel:                [6, 6],
      bias:                       [6, 6],
      overDrive:                  [0, 0],
      lpf:                        [false, false],
      bypass:                     [false],
      link:                       [true]
    },
    wet: 100
  },
  'distopik_bwl_1': {
    channel_ids: ['0', '1'],
    parameters: {
      input:                      [5, 5],
      output:                     [5, 5],
      ceiling:                    [18],
      fet:                        ['JFET'],
      link:                       [true],
      bypass:                     [false]
    },
    wet: 100
  },
  'distopik_eqp1_1': {
    channel_ids: ['0'],
    parameters: {
      lowBoost:                   [0, 0],
      lowAttenuation:             [0, 0],
      lowFrequency:               [20, 20],
      midBoost:                   [0, 0],
      midAttenuation:             [0, 0],
      midFrequency:               [400, 400],
      highBoost:                  [0, 0],
      highAttenuation:            [0, 0],
      highFrequency:              [3000, 3000],
      attenuationSelection:       [5, 5],
      bandwidth:                  [0, 0],
      filter:                     [true],
      link:                       [false]
    },
    wet: 100
  },
  'distopik_fc670_1': {
    channel_ids: ['0', '1'],
    parameters: {
      metering:                   ['GR'],
      input:                      [-10, -10],
      threshold:                  [-5, -5],
      ratio:                      [4, 4],
      timeConstant:               [1, 1],
      midSideEnabled:             [false],
      linkSideChain:              [false],
      link:                       [true]
    },
    wet: 100
  },
  'distopik_la2a_1': {
    channel_ids: ['0', '1'],
    parameters: {
      feedback:                   [50, 50],
      gain:                       [50, 50],
      peakReduction:              [50, 50],
      emphasis:                   [0, 0],
      link:                       [true],
      bypass:                     [false],
      metering:                   ['GR'],
      speed:                      ['SLOW', 'SLOW'],
      mode:                       ['COMPRESS', 'COMPRESS']
    },
    wet: 100
  },
  'distopik_meq_1': {
    channel_ids: ['0', '1'],
    parameters: {
      lowQ:                       [0, 0],
      lowFrequency:               [22, 0],
      lowGain:                    [0, 0],
      lowDoubleGain:              [5, 5],
      lowEnabled:                 ['IN', 'IN'],

      lowMidQ:                    [0, 0],
      lowMidFrequency:            [130, 130],
      lowMidGain:                 [0, 0],
      lowMidDoubleGain:           [5, 5],
      lowMidEnabled:              ['IN', 'IN'],

      midQ:                       [0, 0],
      midFrequency:               [220, 220],
      midGain:                    [0, 0],
      midDoubleGain:              [5, 5],
      midEnabled:                 ['IN', 'IN'],

      highMidQ:                   [0, 0],
      highMidFrequency:           [480, 480],
      highMidGain:                [0, 0],
      highMidDoubleGain:          [5, 5],
      highMidEnabled:             ['IN', 'IN'],

      highSlope:                  [true, true],
      highFrequency:              [900, 900],
      highGain:                   [0, 0],
      highDoubleGain:             [5, 5],
      highEnabled:                ['IN', 'IN'],

      midSideEnabled:             [false, false],
      link:                       [true],
      bypass:                     [false]
    },
    wet: 100
  },
  'distopik_vca_1': {
    channel_ids: ['0', '1'],
    parameters: {
      threshold:                  [0, 0],
      attack:                     [1, 1],
      ratio:                      [4, 4],
      release:                    [0.01, 0.01],
      makeUp:                     [0, 0],
      scf:                        ['OFF', 'OFF'],
      scLink:                     [0, 0],
      link:                       [true],
      bypass:                     [false]
    },
    wet: 100
  },
  'elysia_karacter_1': {
    channel_ids: ['0', '1'],
    parameters: {
      drive:                      [5, 5],
      color:                      [6, 6],
      gain:                       [1, 1],
      mix:                        [110, 110],
      enabled:                    [true, true],
      fetShred:                   [false, false],
      turboBoost:                 [false, false],
      link:                       [true],
      midSideEnabled:             [false, false]
    },
    wet: 100
  },
  'elysia_museq_1': {
    channel_ids: ['0', '1'],
    parameters: {

      lowGain:                    [0, 0],
      lowFreq:                    [9, 9],
      lowCutCurve:                [false, false],
      lowCutGain:                 [false, false],

      bottomGain:                 [0, 0],
      bottomFreq:                 [9, 9],
      bottomHighQ:                [false, false],
      bottomCutGain:              [false, false],

      middleGain:                 [0, 0],
      middleFreq:                 [9, 9],
      middleHighQ:                [false, false],
      middleCutGain:              [false, false],

      topGain:                    [0, 0],
      topFreq:                    [9, 9],
      topHighQ:                   [false, false],
      topCutGain:                 [false, false],

      highGain:                   [0, 0],
      highFreq:                   [9, 9],
      highCutCurve:               [false, false],
      highCutGain:                [false, false],

      enabled:                    [true, true],
      warmMode:                   [false, false],

      link:                       [true]
    },
    wet: 100
  },
  'elysia_nvelope_1': {
    channel_ids: ['0', '1'],
    parameters: {
      attack:                     [0, 0],
      freqA:                      [20, 20],
      sustain:                    [1, 1],
      freqS:                      [50, 50],
      lrOn:                       [true, true],
      eqMode:                     [false, false],
      fullRange:                  [false, false],
      link:                       [false],
      autoGain:                   [false, false]
    },
    wet: 100
  },
  'elysia_xfilter_1': {
    channel_ids: ['0'],
    parameters: {
      lowGain:                    [0],
      lowFreq:                    [20],
      lowMidGain:                 [0],
      lowMidFreq:                 [45],
      highMidGain:                [0],
      highMidFreq:                [300],
      highGain:                   [0],
      highFreq:                   [700],
      hitIt:                      [false],
      lowCut:                     [false],
      lowMidNarrowQ:              [false],
      highMidNarrowQ:             [false],
      highCut:                    [false],
      passiveMassage:             [false]
    },
    wet: 100
  },
  'elysia_xpressor_1': {
    channel_ids: ['0'],
    parameters: {
      threshold:                  [0],
      attack:                     [10],
      release:                    [0],
      ratio:                      [0],
      scf:                        [31],
      grl:                        [30],
      gain:                       [0],
      mix:                        [0],
      hitIt:                      [false],
      warmMode:                   [false],
      logRel:                     [false],
      autoFast:                   [false]
    },
    wet: 100
  },
  'gyraf_g24_1': {
    channel_ids: ['0', '1'],
    parameters: {
      threshold:                  [0, 0],
      ratio:                      [0, 0],
      attack:                     [0, 0],
      release:                    [0, 0],
      feed:                       [0, 0],
      elliptic:                   [0, 0],
      control:                    [0],
      emphasis:                   [0],
      output:                     [0],
      outputType:                 ['ACTIVE'],
      link:                       [false]
    },
    wet: 100
  },
  'neve_1084_1': {
    channel_ids: ['0'],
    parameters: {
      inputGain:                  ['OFF', 'OFF'],
      highShelfGain:              [0, 0],
      highShelfFreq:              [0, 0],
      highBellGain:               [0, 0],
      highBellFreq:               [0, 0],
      highBellHIQ:                [false, false],
      lowBellGain:                [0, 0],
      lowBellFreq:                [0, 0],
      lowBellHIQ:                 [false, false],
      lowShelfGain:               [0, 0],
      lowShelfFreq:               [0, 0],
      loweShelfLOZ:               [false, false],
      hpf:                        ['OFF', 'OFF'],
      eql:                        [true, true],
      link:                       [true]
    },
    wet: 100
  },
  'tierra_gravity_1': {
    channel_ids: ['0'],
    parameters: {
      threshold:                  [0],
      ratio:                      [4],
      attack:                     [4],
      release:                    [2],
      makeUp:                     [0],
      bypass:                     [false],
      highpass:                   [false]
    },
    wet: 100
  }
}