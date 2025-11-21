import { ethers } from "ethers";
import { gigMarketplaceABI } from "./abi/gigMarketplaceABI.js";
const GIG_MARKET_PLACE_ADDRESS = "0x78b06bfd164Ae7dee46E606da3e4d2Cb59997cD2";
export const getGigContract = (providerOrSigner) => new ethers.Contract(GIG_MARKET_PLACE_ADDRESS, gigMarketplaceABI, providerOrSigner);
//# sourceMappingURL=contracts.js.map