<script lang="ts">
  import { onMount } from 'svelte';
  import { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';
  import { WebLinksAddon } from 'xterm-addon-web-links';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { open } from '@tauri-apps/plugin-shell';
  import 'xterm/css/xterm.css';

  let { theme, fontFamily = 'JetBrains Mono', autoCopy = true } = $props();

  let terminalElement: HTMLDivElement;
  let term: Terminal;
  let fitAddon: FitAddon;
  let scrollPosition = $state(0);
  let targetScroll = $state(0);

  export function focus() {
    term?.focus();
  }

  onMount(async () => {
    fitAddon = new FitAddon();
    term = new Terminal({
      cursorBlink: true,
      fontFamily: `${fontFamily}, monospace`,
      theme: {
        background: 'transparent',
        foreground: theme?.foreground || '#ededed',
        cursor: theme?.cursor || '#ffffff',
        selectionBackground: 'rgba(255, 255, 255, 0.1)',
        black: theme?.black || '#000000',
        red: theme?.red || '#ff5555',
        green: theme?.green || '#50fa7b',
        yellow: theme?.yellow || '#f1fa8c',
        blue: theme?.blue || '#bd93f9',
        magenta: theme?.magenta || '#ff79c6',
        cyan: theme?.cyan || '#8be9fd',
        white: theme?.white || '#bfbfbf',
      },
      allowTransparency: true,
    });

    term.loadAddon(fitAddon);
    term.loadAddon(new WebLinksAddon((event, url) => {
      open(url);
    }));
    
    term.open(terminalElement);
    fitAddon.fit();

    const damping = 0.15;
    function smoothScroll() {
      if (Math.abs(targetScroll - scrollPosition) > 0.1) {
        scrollPosition += (targetScroll - scrollPosition) * damping;
        term.scrollToLine(Math.round(scrollPosition));
      }
      requestAnimationFrame(smoothScroll);
    }
    requestAnimationFrame(smoothScroll);

    term.onScroll((newPos) => {
      targetScroll = newPos;
    });

    const unlisten = await listen('pty-data', (event: any) => {
      term.write(event.payload);
      targetScroll = term.buffer.active.baseY;
    });

    term.onData((data) => {
      invoke('write_to_pty', { data });
    });

    term.onSelectionChange(() => {
      if (autoCopy && term.hasSelection()) {
        const selection = term.getSelection();
        if (selection) {
          navigator.clipboard.writeText(selection);
        }
      }
    });

    const resizeObserver = new ResizeObserver(() => {
      fitAddon.fit();
      invoke('resize_pty', { 
        rows: term.rows, 
        cols: term.cols 
      });
    });
    resizeObserver.observe(terminalElement);

    term.focus();

    return () => {
      unlisten();
      resizeObserver.disconnect();
      term.dispose();
    };
  });
</script>

<div class="terminal-shell">
  <div class="terminal-viewport" bind:this={terminalElement}></div>
  <div class="glow-edge"></div>
</div>

<style>
  .terminal-shell {
    position: relative;
    height: 100%;
    width: 100%;
    overflow: hidden;
    background: transparent;
    box-sizing: border-box;
    transform: translateZ(0);
    padding: 12px;
  }

  .terminal-viewport {
    height: 100%;
    width: 100%;
    overflow: hidden;
  }

  .glow-edge {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.05), transparent);
  }

  :global(.xterm) {
    height: 100% !important;
  }

  :global(.xterm-viewport) {
    background: transparent !important;
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
  }

  :global(.xterm-rows) {
    font-size: clamp(14px, 1.2vw, 18px) !important;
    line-height: 1.5 !important;
  }

  /* Custom Modern Scrollbar */
  :global(.xterm-viewport::-webkit-scrollbar) {
    width: 4px;
  }
  :global(.xterm-viewport::-webkit-scrollbar-thumb) {
    background: rgba(255, 255, 255, 0.12);
    border-radius: 10px;
    transition: background 0.3s;
  }
  :global(.xterm-viewport::-webkit-scrollbar-thumb:hover) {
    background: rgba(255, 255, 255, 0.2);
  }
</style>
