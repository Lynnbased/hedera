// import { SUPPORTED_CHAIN_ID } from "@/context";

// export const isSupportedChain = (
//   chainId: number | undefined
// ): chainId is number =>
//   chainId !== undefined && Number(chainId) === SUPPORTED_CHAIN_ID;

import { defineChain } from "thirdweb";

const THIRDWEB_API_KEY = process.env.THIRDWEB_CLIENT_ID;

// Define Hedera Testnet chain
export const hederaTestnet = defineChain({
  id: 296,
  name: "Hedera Testnet",
  nativeCurrency: {
    name: "Hedera Testnet",
    symbol: "HBar",
    decimals: 18,
  },
  rpc: `https://296.rpc.thirdweb.com/${THIRDWEB_API_KEY}`,
  blockExplorers: [
    {
      name: "Hash Scan",
      url: "https://hashscan.io/testnet",
    },
  ],
  testnet: true,
});

export const SUPPORTED_CHAIN_ID = 296;

export const isSupportedChain = (
  chainId: number | undefined
): chainId is number =>
  chainId !== undefined && Number(chainId) === SUPPORTED_CHAIN_ID;