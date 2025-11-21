// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Registry} from "../src/Registry.sol";
import {Token} from "../src/Token.sol";
import {PaymentProcessor} from "../src/PaymentProcessor.sol";
import {GigMarketplace} from "../src/GigMarketplace.sol";
import {ReviewSystem} from "../src/ReviewSystem.sol";
import {ChatSystem} from "../src/ChatSystem.sol";
import {CraftCoin} from "../src/CraftCoin.sol";

contract DeployScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("HEDERA_PRIVATE_KEY");
        require(deployerPrivateKey != 0, "Deployer private key is not set");

        address relayer = vm.addr(deployerPrivateKey);
        // address relayer = vm.envAddress("RELAYER");

        vm.startBroadcast(deployerPrivateKey);
        Registry registry = new Registry(relayer);
        Token token = new Token(relayer);
        CraftCoin craftCoin = new CraftCoin(relayer, address(registry));
        PaymentProcessor paymentProcessor = new PaymentProcessor(relayer, address(token));
        GigMarketplace gigMarketplace =
            new GigMarketplace(relayer, address(registry), address(paymentProcessor), address(craftCoin));
        ReviewSystem reviewSystem = new ReviewSystem(relayer, address(registry), address(gigMarketplace));
        ChatSystem chatSystem = new ChatSystem(address(gigMarketplace));

        writeAddressesToFile(address(registry), "Registry");
        writeAddressesToFile(address(token), "CraftLinkToken");
        writeAddressesToFile(address(craftCoin), "CraftCoinToken");
        writeAddressesToFile(address(paymentProcessor), "PaymentProcessor");
        writeAddressesToFile(address(gigMarketplace), "GigMarketplace");
        writeAddressesToFile(address(reviewSystem), "ReviewSystem");
        writeAddressesToFile(address(chatSystem), "ChatSystem");
        vm.stopBroadcast();
    }

    function writeAddressesToFile(address addr, string memory text) public {
        string memory filename = "./deployed_contracts.txt";

        vm.writeLine(
            filename,
            "---------------------------------------------------------------------------------------------------------------------------"
        );
        vm.writeLine(filename, text);
        vm.writeLine(filename, vm.toString(addr));
        vm.writeLine(
            filename,
            "---------------------------------------------------------------------------------------------------------------------------"
        );
    }
}

// DEPLOY COMMAND
// forge script script/Deploy.s.sol:DeployScript --rpc-url testnet --broadcast --slow

// # Replace with the contract address from the previous step
// export CONTRACT_ADDRESS=<your-contract-address>

// # Derive your public address from the private key
// export MY_ADDRESS=$(cast wallet address $HEDERA_PRIVATE_KEY)

// VERIFY COMMAND WITH CONSTRUCTOR ARGS
// forge verify-contract 0x6182AfDA6817a0a78a5e87Da57DD19F05bfCa9cf src/ReviewSystem.sol:ReviewSystem \
//     --chain-id 296 \
//     --verifier sourcify \
//     --verifier-url "https://server-verify.hashscan.io/" \
//     --constructor-args $(cast abi-encode "constructor(address, address)" 0x3E7dfBF99f10402E860Df4e7420217EF56e94cc1 0x9c78Bbfc9a101f0C467560BD93401B72cC4152C1 0x78b06bfd164Ae7dee46E606da3e4d2Cb59997cD2)

// VERIFY COMMAND WITHOUT CONSTRUCTOR ARGS
// forge verify-contract $CONTRACT_ADDRESS src/HederaToken.sol:HederaToken \
//     --chain-id 296 \
//     --verifier sourcify \
//     --verifier-url "https://server-verify.hashscan.io/"

// Reference for Hedera + Foundry
// https://docs.hedera.com/hedera/getting-started-evm-developers/deploy-a-smart-contract-with-foundry