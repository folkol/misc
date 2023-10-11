/**
 * Test cases from Appendix B
 */
// import { it, expect } from 'jest';
// import { shiftRow, subBytes } from './aes';

const { subBytes, shiftRow } = require('./aes.js');

it('subBytes', () => {
  let state = [
    [0x19, 0xa0, 0x9a, 0xe9],
    [0x3d, 0xf4, 0xc6, 0xf8],
    [0xe3, 0xe2, 0x8d, 0x48],
    [0xbe, 0x2b, 0x2a, 0x08],
  ];

  subBytes(state);

  expect(state).toEqual([
    [0xd4, 0xe0, 0xb8, 0x1e],
    [0x27, 0xbf, 0xb4, 0x41],
    [0x11, 0x98, 0x5d, 0x52],
    [0xae, 0xf1, 0xe5, 0x30],
  ]);
});

it('shiftRow 0', () => {
  let state = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [8, 9, 10, 11],
    [12, 13, 14, 15],
  ];

  shiftRow(state, 0);

  expect(state).toEqual([
      [0, 1, 2, 3],
      [4, 5, 6, 7],
      [8, 9, 10, 11],
      [12, 13, 14, 15],
    ],
  );
});

it('shiftRow 3', () => {
  let state = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [8, 9, 10, 11],
    [12, 13, 14, 15],
  ];

  shiftRow(state, 3);

  expect(state).toEqual([
      [0, 1, 2, 3],
      [4, 5, 6, 7],
      [8, 9, 10, 11],
      [15, 12, 13, 14],
    ],
  );
})
// it('shiftRows', () => {
//     let state = [
//         [0xd4, 0xe0, 0xb8, 0x1e],
//         [0x27, 0xbf, 0xb4, 0x41],
//         [0x11, 0x98, 0x5d, 0x52],
//         [0xae, 0xf1, 0xe5, 0x30],
//     ];
//
//     shiftRows(state);
//
//     expect(state).toEqual([
//         [0xd4, 0xe0, 0xb8, 0x1e],
//         [0xbf, 0xb4, 0x41, 0x27],
//         [0x5d, 0x52, 0x11, 0x98],
//         [0x30, 0xae, 0xf1, 0xe5],
//     ])
// })
