<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { readText } from '@tauri-apps/plugin-clipboard-manager';
  import { Terminal as TerminalIcon, Settings as SettingsIcon, Move, X, HelpCircle, ArrowRight } from 'lucide-svelte';
  import Terminal from '../lib/Terminal.svelte';
  import Settings from '../lib/Settings.svelte';
  import MovementMode from '../lib/MovementMode.svelte';
  import '../app.css';

  let config = $state({
    toggle_key: "Ctrl+`",
    opacity: 0.85,
    blur: true,
    font_size: 14,
    font_family: "JetBrains Mono",
    current_theme: "Minimal Dark",
    themes: [],
    window: { width: 800, height: 500, always_on_top: true }
  });
  
  let showSettings = $state(false);
  let movementMode = $state(false);
  let kbdText = $state("?");
  let clipboardPath = $state<string | null>(null);
  let terminalComponent: Terminal | undefined = $state();

  async function checkClipboard() {
    try {
      const text = await readText();
      if (text && (text.includes('\\') || text.includes('/'))) {
        // Very basic path detection
        clipboardPath = text.trim().replace(/^"|"$/g, '');
      } else {
        clipboardPath = null;
      }
    } catch {
      clipboardPath = null;
    }
  }

  async function jumpToClipboard() {
    if (clipboardPath) {
      await invoke('write_to_pty', { data: `cd "${clipboardPath}"\r` });
      clipboardPath = null;
    }
  }

  let activeTheme = $derived(
    config.themes.find(t => t.name === config.current_theme) || config.themes[0]
  );

  onMount(async () => {
    const savedConfig = await invoke('get_settings');
    if (savedConfig) config = savedConfig as any;
    
    // Auto-focus terminal whenever window gains focus
    const unlistenFocus = await listen('tauri://focus', () => {
      terminalComponent?.focus();
      checkClipboard();
    });
    
    // Also focus on initial load
    setTimeout(() => terminalComponent?.focus(), 100);
    
    window.addEventListener('keydown', (e) => {
      // Smart Snapping (Ctrl + Arrows)
      if (e.ctrlKey) {
        if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(e.key)) {
          e.preventDefault();
          e.stopPropagation();
          
          const step = e.shiftKey ? 100 : 20;
          let dx = 0, dy = 0;
          
          if (e.key === 'ArrowLeft') dx = -step;
          if (e.key === 'ArrowRight') dx = step;
          if (e.key === 'ArrowUp') dy = -step;
          if (e.key === 'ArrowDown') dy = step;
          
          invoke('move_window', { dx, dy });
          return;
        }

      }

      // Block standard browser shortcuts (F12, Ctrl+R, Ctrl+Shift+I)
      if (
        e.key === 'F12' || 
        (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 'i') ||
        (e.ctrlKey && e.key.toLowerCase() === 'r')
      ) {
        e.preventDefault();
        e.stopPropagation();
        return;
      }

      if (e.ctrlKey && e.key === ',') {
        e.preventDefault();
        showSettings = !showSettings;
        return;
      }

      if (e.ctrlKey && e.key.toLowerCase() === 'm') {
        e.preventDefault();
        movementMode = true;
      }
      if (e.key === 'Escape') {
        if (showSettings) showSettings = false;
        if (movementMode) movementMode = false;
      }
    }, { capture: true });
  });

  async function handleSave(newConfig: any) {
    const oldKey = config.toggle_key;
    const oldFontSize = config.font_size;
    config = newConfig;
    await invoke('save_settings', { config });
    
    if (oldKey !== config.toggle_key) {
      await invoke('update_toggle_key', { newKey: config.toggle_key });
    }

    if (oldFontSize !== config.font_size) {
      await invoke('resize_window_to_scale', { fontSize: config.font_size });
    }
  }
</script>

<main class="window-container" style="--window-opacity: {config.opacity}">
  <div class="header">
    <div class="header-title">
      <TerminalIcon size={14} />
      <span>Veil</span>
      <span 
        class="kbd" 
        role="status"
        aria-label="Toggle Keybind"
        onmouseenter={() => kbdText = config.toggle_key}
        onmouseleave={() => kbdText = "?"}
      >
        {#if kbdText === "?"}
          <HelpCircle size={12} />
        {:else}
          {kbdText}
        {/if}
      </span>
    </div>
    
    <div class="header-actions">
      {#if clipboardPath}
        <button 
          class="jump-btn" 
          onclick={jumpToClipboard}
          title="Jump to: {clipboardPath}"
        >
          <ArrowRight size={14} />
          <span>Jump to folder</span>
        </button>
      {/if}
      <button onclick={() => movementMode = true} title="Movement Mode (Ctrl+M)">
        <Move size={14} />
      </button>
      <button onclick={() => showSettings = true} title="Settings">
        <SettingsIcon size={14} />
      </button>
    </div>
  </div>

    <Terminal 
      bind:this={terminalComponent} 
      theme={activeTheme} 
      fontFamily={config.font_family}
      autoCopy={config.auto_copy}
    />

  <Settings 
    bind:show={showSettings} 
    {config} 
    onSave={handleSave} 
  />
  
  <MovementMode 
    bind:active={movementMode} 
    onExit={() => movementMode = false} 
  />
</main>

<style>
  :global(:root) {
    --window-opacity: 0.85;
  }
</style>
