import { Injectable, effect, signal } from '@angular/core';

import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { SolflareWalletAdapter } from '@solana/wallet-adapter-solflare';
import { CoinbaseWalletAdapter } from '@solana/wallet-adapter-coinbase';
import { LedgerWalletAdapter } from '@solana/wallet-adapter-ledger';
import { TrustWalletAdapter } from '@solana/wallet-adapter-trust';
import { MathWalletAdapter } from '@solana/wallet-adapter-mathwallet';
import { AlphaWalletAdapter } from '@solana/wallet-adapter-alpha';
import { AvanaWalletAdapter } from '@solana/wallet-adapter-avana';
import { BitpieWalletAdapter } from '@solana/wallet-adapter-bitpie';
import { CloverWalletAdapter } from '@solana/wallet-adapter-clover';
import { Coin98WalletAdapter } from '@solana/wallet-adapter-coin98';
import { CoinhubWalletAdapter } from '@solana/wallet-adapter-coinhub';
import { FractalWalletAdapter } from '@solana/wallet-adapter-fractal';
import { HuobiWalletAdapter } from '@solana/wallet-adapter-huobi';
import { HyperPayWalletAdapter } from '@solana/wallet-adapter-hyperpay';
import { KrystalWalletAdapter } from '@solana/wallet-adapter-krystal';
import { NekoWalletAdapter } from '@solana/wallet-adapter-neko';
import { NightlyWalletAdapter } from '@solana/wallet-adapter-nightly';
import { NufiWalletAdapter } from '@solana/wallet-adapter-nufi';
import { OntoWalletAdapter } from '@solana/wallet-adapter-onto';
import { ParticleAdapter } from '@solana/wallet-adapter-particle';
import { SafePalWalletAdapter } from '@solana/wallet-adapter-safepal';
import { SaifuWalletAdapter } from '@solana/wallet-adapter-saifu';
import { SalmonWalletAdapter } from '@solana/wallet-adapter-salmon';
import { SkyWalletAdapter } from '@solana/wallet-adapter-sky';
import { SolongWalletAdapter } from '@solana/wallet-adapter-solong';
import { SpotWalletAdapter } from '@solana/wallet-adapter-spot';
import { TokenaryWalletAdapter } from '@solana/wallet-adapter-tokenary';
import { TokenPocketWalletAdapter } from '@solana/wallet-adapter-tokenpocket';
import { TrezorWalletAdapter } from '@solana/wallet-adapter-trezor';
import { UnsafeBurnerWalletAdapter } from '@solana/wallet-adapter-unsafe-burner';
import { XDEFIWalletAdapter } from '@solana/wallet-adapter-xdefi';

import { WalletAdapter, WalletReadyState } from '@solana/wallet-adapter-base';

@Injectable({
  providedIn: 'root',
})
export class WalletService {
  readonly all_wallets: WalletAdapter[] = [
    new PhantomWalletAdapter(),
    new SolflareWalletAdapter(),
    new AlphaWalletAdapter(),
    new AvanaWalletAdapter(),
    new BitpieWalletAdapter(),
    new CloverWalletAdapter(),
    new Coin98WalletAdapter(),
    new CoinbaseWalletAdapter(),
    new LedgerWalletAdapter(),
    new TrustWalletAdapter(),
    new MathWalletAdapter(),
    new CoinhubWalletAdapter(),
    new FractalWalletAdapter(),
    new HuobiWalletAdapter(),
    new HyperPayWalletAdapter(),
    new KrystalWalletAdapter(),
    new NekoWalletAdapter(),
    new NightlyWalletAdapter(),
    new NufiWalletAdapter(),
    new OntoWalletAdapter(),
    new ParticleAdapter(),
    new SafePalWalletAdapter(),
    new SaifuWalletAdapter(),
    new SalmonWalletAdapter(),
    new SkyWalletAdapter(),
    new SolongWalletAdapter(),
    new SpotWalletAdapter(),
    new TokenaryWalletAdapter(),
    new TokenPocketWalletAdapter(),
    new TrezorWalletAdapter(),
    new UnsafeBurnerWalletAdapter(),
    new XDEFIWalletAdapter(),
  ];

  wallets = signal<WalletAdapter[]>([]);
  activeWallet = signal<WalletAdapter | null>(null);

  constructor() {
    const priority = {
      [WalletReadyState.Installed]: 0,
      [WalletReadyState.Loadable]: 1,
      [WalletReadyState.NotDetected]: 2,
      [WalletReadyState.Unsupported]: 3,
    };

    const wallets = this.all_wallets.sort(
      (a, b) => priority[a.readyState] - priority[b.readyState]
    );

    this.wallets.set(wallets);

    effect(() => {
      this.activeWallet()?.on('disconnect', () => {
        this.activeWallet.set(null);
        console.log(`${this.activeWallet()?.name} disconnected ❌`);
      });

      this.activeWallet()?.on('error', (error) => {
        console.error(`${this.activeWallet()?.name} error:`, error);
      });
    });
  }

  async connectWallet(wallet: WalletAdapter) {
    await wallet
      .connect()
      .then(() => {
        this.activeWallet.set(wallet);
        console.log(`${wallet.name} connected ✅`);
      })
      .catch((e) => {
        console.error(e);
      });
  }

  async disconnect() {
    await this.activeWallet()?.disconnect();
  }
}
