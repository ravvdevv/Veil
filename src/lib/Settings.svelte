<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { X, Keyboard, Sun, Moon, Maximize2, Plus, Minus } from 'lucide-svelte';

  let { show = $bindable(false), config, onSave } = $props();

  let recording = $state(false);
  let recordedKey = $state("");

  function startRecording() {
    recording = true;
    recordedKey = "Press keys...";
    window.addEventListener('keydown', handleKeyDown);
  }

  function handleKeyDown(e: KeyboardEvent) {
    e.preventDefault();
    e.stopPropagation();

    if (e.key === 'Escape') {
      recording = false;
      window.removeEventListener('keydown', handleKeyDown);
      return;
    }

    const modifiers = [];
    if (e.ctrlKey) modifiers.push('Ctrl');
    if (e.altKey) modifiers.push('Alt');
    if (e.shiftKey) modifiers.push('Shift');
    if (e.metaKey) modifiers.push('Meta');

    let key = e.key;
    if (key === 'Control' || key === 'Alt' || key === 'Shift' || key === 'Meta') {
      recordedKey = modifiers.join('+');
      return;
    }

    // Map some keys to match Tauri's shortcut format
    if (key === '`') key = '`';
    if (key === ' ') key = 'Space';
    
    const finalKey = [...modifiers, key.toUpperCase()].join('+');
    config.toggle_key = finalKey;
    recording = false;
    window.removeEventListener('keydown', handleKeyDown);
    onSave(config);
  }

  function updateOpacity(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    config.opacity = val;
    onSave(config);
  }

  // Dragging logic
  let pos = $state({ x: 0, y: 0 });
  let dragging = $state(false);
  let startPos = { x: 0, y: 0 };

  function handleMouseDown(e: MouseEvent) {
    // Only drag from the header, not buttons
    if ((e.target as HTMLElement).closest('button')) return;
    
    dragging = true;
    startPos = { x: e.clientX - pos.x, y: e.clientY - pos.y };
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!dragging) return;
    pos.x = e.clientX - startPos.x;
    pos.y = e.clientY - startPos.y;
  }

  function handleMouseUp() {
    dragging = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
  }
</script>

{#if show && config}
  <div class="settings-overlay">
    <div 
      class="settings-card" 
      style="transform: translate({pos.x}px, {pos.y}px)"
    >
      <div 
        class="settings-header" 
        onmousedown={handleMouseDown}
        role="button"
        tabindex="0"
      >
        <div class="flex items-center gap-2">
          <Maximize2 size={16} />
          <span class="text-sm font-semibold">Settings</span>
        </div>
        <button onclick={() => show = false}>
          <X size={16} />
        </button>
      </div>

      <div class="settings-body">
        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Toggle Shortcut</span>
            <span class="description">Global key to show/hide Veil</span>
          </div>
          <button 
            class="recorder-btn {recording ? 'recording' : ''}" 
            onclick={startRecording}
          >
            {recording ? recordedKey : config.toggle_key}
          </button>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Window Opacity</span>
            <span class="description">Control background transparency</span>
          </div>
          <input 
            type="range" 
            min="0.1" 
            max="1.0" 
            step="0.05" 
            value={config.opacity} 
            oninput={updateOpacity}
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Theme</span>
            <span class="description">Choose your terminal aesthetic</span>
          </div>
          <select 
            value={config.current_theme} 
            onchange={(e) => {
              config.current_theme = (e.target as HTMLSelectElement).value;
              onSave(config);
            }}
          >
            {#each config.themes as theme}
              <option value={theme.name}>{theme.name}</option>
            {/each}
          </select>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Font Size</span>
            <span class="description">Adjust terminal text size</span>
          </div>
          <div class="stepper">
            <button 
              onclick={() => {
                config.font_size = Math.max(8, config.font_size - 1);
                onSave(config);
              }}
              title="Decrease size"
            >
              <Minus size={14} />
            </button>
            <span class="stepper-value">{config.font_size}px</span>
            <button 
              onclick={() => {
                config.font_size = Math.min(32, config.font_size + 1);
                onSave(config);
              }}
              title="Increase size"
            >
              <Plus size={14} />
            </button>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Font Family</span>
            <span class="description">Select your monospace font</span>
          </div>
          <input 
            type="text" 
            value={config.font_family} 
            oninput={(e) => {
              config.font_family = (e.target as HTMLInputElement).value;
              onSave(config);
            }}
            class="text-input"
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Ghost Mode</span>
            <span class="description">Auto-hide on focus loss</span>
          </div>
          <input 
            type="checkbox" 
            checked={config.auto_hide} 
            onchange={(e) => {
              config.auto_hide = (e.target as HTMLInputElement).checked;
              onSave(config);
            }}
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Auto-copy</span>
            <span class="description">Copy selection instantly</span>
          </div>
          <input 
            type="checkbox" 
            checked={config.auto_copy} 
            onchange={(e) => {
              config.auto_copy = (e.target as HTMLInputElement).checked;
              onSave(config);
            }}
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="label">Always on Top</span>
            <span class="description">Keep window above all others</span>
          </div>
          <input 
            type="checkbox" 
            checked={config.window.always_on_top} 
            onchange={(e) => {
              config.window.always_on_top = (e.target as HTMLInputElement).checked;
              onSave(config);
            }}
          />
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .settings-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 999;
    backdrop-filter: blur(var(--glass-blur));
    animation: fade-in var(--transition-smooth);
    -webkit-app-region: no-drag;
  }

  .settings-card {
    position: relative;
    background: var(--bg-secondary);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    width: clamp(300px, 90vw, 340px);
    max-height: clamp(200px, 80vh, 600px);
    pointer-events: auto;
    box-shadow: var(--shadow-lg);
    animation: slide-up var(--transition-smooth);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .settings-header {
    padding: 14px 18px;
    background: var(--surface-highlight);
    border-bottom: 1px solid var(--border);
    display: flex;
    justify-content: space-between;
    align-items: center;
    cursor: grab;
    user-select: none;
  }

  .settings-header:active {
    cursor: grabbing;
  }

  .settings-body {
    padding: var(--gap-lg);
    display: flex;
    flex-direction: column;
    gap: var(--gap-lg);
    flex: 1;
    overflow-y: auto;
    scrollbar-gutter: stable;
  }

  .settings-body::-webkit-scrollbar {
    width: 4px;
  }

  .settings-body::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 10px;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--gap-md);
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .label {
    font-size: 13px;
    font-weight: 500;
  }

  .description {
    font-size: 11px;
    color: var(--text-secondary);
  }

  .stepper {
    display: flex;
    align-items: center;
    background: var(--surface-highlight);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .stepper button {
    padding: 6px 8px;
    color: var(--text-secondary);
    transition: all var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stepper button:hover {
    background: var(--surface-highlight);
    color: var(--text-primary);
  }

  .stepper-value {
    font-family: var(--font-mono);
    font-size: 12px;
    padding: 0 8px;
    min-width: 40px;
    text-align: center;
    border-left: 1px solid var(--border);
    border-right: 1px solid var(--border);
  }

  .label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .description {
    font-size: 11px;
    color: var(--text-secondary);
  }

  .flex { display: flex; }
  .items-center { align-items: center; }
  .gap-2 { gap: 8px; }
  .text-sm { font-size: 14px; }
  .font-semibold { font-weight: 600; }

  input[type="range"],
  input[type="checkbox"] {
    accent-color: var(--accent);
    cursor: pointer;
  }

  select, .text-input {
    background: var(--surface-highlight);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    padding: 4px 8px;
    font-size: 12px;
    outline: none;
    transition: all var(--transition-fast);
  }

  select:hover, .text-input:hover {
    background: var(--border);
  }

  .text-input:focus {
    border-color: var(--accent);
    background: var(--surface-highlight);
  }

  .text-input {
    width: 140px;
    font-family: var(--font-mono);
  }

  option {
    background: var(--bg-primary);
    color: var(--text-primary);
  }
</style>
