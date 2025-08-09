import { NgFor, NgIf } from '@angular/common';
import { Component, inject, signal } from '@angular/core';
import { WalletService } from '../wallet.service';
import { WalletReadyState } from '@solana/wallet-adapter-base';

@Component({
  selector: 'app-navbar',
  imports: [NgIf, NgFor],
  templateUrl: './navbar.component.html',
  styleUrl: './navbar.component.css',
})
export class NavbarComponent {
  walletService = inject(WalletService);
  wallets = this.walletService.wallets;
  activeWallet = this.walletService.activeWallet;
  openWallet = signal(false);
  readonly WalletReadyState = WalletReadyState;

  openWallets() {
    this.openWallet.set(!this.openWallet());
  }

  connectWallet(name: string) {
    const wallet = this.walletService.wallets().find((w) => w.name === name);
    if (!wallet) return;

    if (wallet.readyState === WalletReadyState.Installed) {
      this.walletService.connectWallet(wallet);
      this.openWallet.set(false);
    } else {
      window.open(wallet.url, '_blank');
    }
  }

  disconnectWallet() {
    this.walletService.disconnect();
  }
}
