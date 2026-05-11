<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { Move } from 'lucide-svelte';

  let { active = $bindable(false), onExit } = $props();

  const appWindow = getCurrentWindow();
  const MOVE_STEP = 10;
  const RESIZE_STEP = 10;

  async function handleKeyDown(e: KeyboardEvent) {
    if (!active) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      onExit();
      return;
    }

    if (!['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
      return;
    }

    e.preventDefault();
    e.stopPropagation();

    const step = e.shiftKey ? RESIZE_STEP : MOVE_STEP;

    if (e.shiftKey) {
      // Resize mode
      let dw = 0, dh = 0;
      switch (e.key) {
        case 'ArrowUp': dh = -step; break;
        case 'ArrowDown': dh = step; break;
        case 'ArrowLeft': dw = -step; break;
        case 'ArrowRight': dw = step; break;
      }
      await invoke('resize_window', { dw, dh });
    } else {
      // Move mode
      let dx = 0, dy = 0;
      switch (e.key) {
        case 'ArrowUp': dy = -step; break;
        case 'ArrowDown': dy = step; break;
        case 'ArrowLeft': dx = -step; break;
        case 'ArrowRight': dx = step; break;
      }
      await invoke('move_window', { dx, dy });
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeyDown, { capture: true });
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeyDown, { capture: true });
  });
</script>

{#if active}
  <div class="movement-overlay">
    <div class="movement-indicator">
      <Move size={24} />
      <span>Movement Mode</span>
      <div class="hints">
        <span class="hint"><kbd>Arrows</kbd> Move</span>
        <span class="hint"><kbd>Shift + Arrows</kbd> Resize</span>
        <span class="hint"><kbd>Esc</kbd> Exit</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .movement-overlay {
    position: fixed;
    inset: 0;
    z-index: 200;
    background: rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }

  .movement-indicator {
    background: rgba(15, 15, 15, 0.8);
    backdrop-filter: blur(12px);
    padding: 24px;
    border-radius: var(--radius-lg);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.1);
    animation: scale-in 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes scale-in {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }

  .hints {
    display: flex;
    gap: 16px;
    margin-top: 12px;
  }

  .hint {
    font-size: 11px;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  kbd {
    background: rgba(255, 255, 255, 0.1);
    padding: 2px 4px;
    border-radius: 3px;
    font-family: 'JetBrains Mono', monospace;
  }
</style>
