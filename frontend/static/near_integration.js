let nearAPI;

async function loadNearApi() {
    if (!nearAPI) {
      nearAPI = await import('https://cdn.jsdelivr.net/npm/near-api-js/dist/near-api-js.min.js');
    }
  }

async function startGame() {
    if (!nearAPI) {
      nearAPI = await import('https://cdn.jsdelivr.net/npm/near-api-js/dist/near-api-js.min.js');
    }
}

async function initContract() {


    await loadNearApi();
    const { connect, WalletConnection } = nearAPI;
    // Initialize connection to NEAR
    const near = await connect({
        networkId: "testnet",
        keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: "https://rpc.testnet.near.org",
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        explorerUrl: "https://explorer.testnet.near.org",
    });
    const walletConnection = new WalletConnection(near);
    
    window.walletConnection = walletConnection;
}

async function signIn() {
    window.walletConnection.requestSignIn("pbc2024.testnet"); 
}

export { initContract, signIn, startGame };