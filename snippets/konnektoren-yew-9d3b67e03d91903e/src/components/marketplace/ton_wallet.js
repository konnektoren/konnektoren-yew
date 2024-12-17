import { toUserFriendlyAddress } from "https://esm.run/@tonconnect/sdk";
import { TonConnectUI } from "https://esm.run/@tonconnect/ui";

const USE_TEST_NETWORK = true;
const TON_API_URL = "https://testnet.tonapi.io/v2";

let tonConnectUI = null;

export async function initTonWallet(
  manifestUrl,
  onConnectCallback,
  onDisconnectCallback,
) {
  try {
    if (!tonConnectUI) {
      tonConnectUI = new TonConnectUI({
        manifestUrl: manifestUrl,
        buttonRootId: "ton-wallet-button",
      });
    }

    tonConnectUI.onStatusChange(async (wallet) => {
      try {
        if (!wallet) {
          onDisconnectCallback("Not connected");
          return;
        }
        const address = wallet.account.address;
        const balance = await getWalletBalance(address);
        onConnectCallback(address, balance.toString());
      } catch (innerError) {
        console.error("Error in onStatusChange callback:", innerError);
        onConnectCallback("Error", "0");
      }
    });

    // Return the tonConnectUI instance
    return tonConnectUI;
  } catch (outerError) {
    console.error("Error in initTonWallet:", outerError);
    onConnectCallback("Error", "0");
    return null;
  }
}

async function getWalletBalance(address) {
  // Implement this function to fetch the wallet balance
  // You might need to use a TON API or SDK for this
  // For now, we'll return a placeholder value
  return 1000000000; // 1 TON
}

export async function payTonWallet(address, amount) {
  if (!tonConnectUI) {
    console.error("TonConnect UI is not initialized");
    throw new Error("TonConnect UI is not initialized");
  }

  if (!tonConnectUI.account) {
    console.error("No account connected");
    throw new Error("No account connected");
  }

  try {
    // In TON, 1 TON = 1,000,000,000 nanoTONs
    // The amount should be provided in nanoTONs
    // For example:
    // 1 TON = 1,000,000,000
    // 0.1 TON = 100,000,000
    // 0.01 TON = 10,000,000
    //
    const nanoTonAmount = amount.toString();
    const userFriendlyAddress = toUserFriendlyAddress(
      address,
      USE_TEST_NETWORK,
    );

    const transaction = {
      validUntil: Math.floor(new Date() / 1000) + 360,
      messages: [
        {
          address: userFriendlyAddress,
          amount: nanoTonAmount,
        },
      ],
    };

    return await tonConnectUI.sendTransaction(transaction);
  } catch (error) {
    console.error("Error sending transaction:", error);
    if (error instanceof Error) {
      console.error("Error details:", {
        name: error.name,
        message: error.message,
        stack: error.stack,
      });
    }
    throw error;
  }
}
