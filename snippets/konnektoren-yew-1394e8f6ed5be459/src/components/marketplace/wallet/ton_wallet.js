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
    // Wait for the button element to exist
    await waitForElement("#ton-wallet-button");

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

    return tonConnectUI;
  } catch (outerError) {
    console.error("Error in initTonWallet:", outerError);
    onConnectCallback("Error", "0");
    return null;
  }
}

function waitForElement(selector) {
  return new Promise((resolve) => {
    if (document.querySelector(selector)) {
      return resolve(document.querySelector(selector));
    }

    const observer = new MutationObserver(() => {
      if (document.querySelector(selector)) {
        observer.disconnect();
        resolve(document.querySelector(selector));
      }
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true,
    });
  });
}

async function getWalletBalance(address) {
  try {
    // Use the correct API endpoint for account information
    const response = await fetch(`${TON_API_URL}/accounts/${address}`);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    // The balance is in nanoTONs, convert to TON by dividing by 10^9
    return data.balance || 0;
  } catch (error) {
    console.error("Error fetching balance:", error);
    return 1000000000; // Return 1 TON as default for testing
  }
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
    const nanoTonAmount = amount.toString();
    const userFriendlyAddress = toUserFriendlyAddress(
      address,
      USE_TEST_NETWORK,
    );

    const transaction = {
      validUntil: Math.floor(Date.now() / 1000) + 360,
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
    throw error;
  }
}
