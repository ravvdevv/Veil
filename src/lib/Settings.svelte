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
      <div class="settings-header" onmousedown={handleMouseDown}>
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
    z-index: 999; /* Ensure it's above everything */
    backdrop-filter: blur(8px);
    animation: fade-in 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    -webkit-app-region: no-drag; /* Stop window dragging while menu is open */
  }

  .settings-card {
    position: relative;
    background: rgba(20, 20, 20, 0.85);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: var(--radius-lg);
    
    /* Equation-based sizing */
    width: clamp(300px, 90vw, 340px);
    max-height: clamp(200px, 80vh, 600px);
    
    pointer-events: auto;
    box-shadow: 
      0 0 0 1px rgba(255, 255, 255, 0.05),
      0 20px 40px rgba(0, 0, 0, 0.4),
      0 10px 20px rgba(0, 0, 0, 0.2);
    animation: slide-up 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  @keyframes slide-up {
    from { opacity: 0; transform: translateY(10px) scale(0.98); }
    to { opacity: 1; transform: translateY(0) scale(1); }
  }

  @keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .settings-header {
    padding: 14px 18px;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
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
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    flex: 1;
    overflow-y: auto;
    scrollbar-gutter: stable;
  }

  .settings-body::-webkit-scrollbar {
    width: 4px;
  }

  .settings-body::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 2px;
  }

  .stepper {
    display: flex;
    align-items: center;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .stepper button {
    padding: 6px 8px;
    color: var(--text-secondary);
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stepper button:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-primary);
  }

  .stepper-value {
    font-family: 'JetBrains Mono', monospace;
    font-size: 12px;
    padding: 0 8px;
    min-width: 40px;
    text-align: center;
    border-left: 1px solid var(--border);
    border-right: 1px solid var(--border);
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
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

  .recorder-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border);
    padding: 6px 12px;
    font-family: 'JetBrains Mono', monospace;
    font-size: 12px;
    min-width: 100px;
  }

  .recorder-btn.recording {
    border-color: #fff;
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .flex { display: flex; }
  .items-center { align-items: center; }
  .gap-2 { gap: 8px; }
  .text-sm { font-size: 14px; }
  .font-semibold { font-weight: 600; }

  input[type="range"] {
    width: 100px;
    accent-color: var(--accent);
  }

  input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--accent);
  }

  select {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    padding: 4px 8px;
    font-size: 12px;
    outline: none;
    cursor: pointer;
  }

  select:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  option {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .number-input, .text-input {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    padding: 4px 8px;
    font-size: 12px;
    outline: none;
    width: 100px;
  }

  .text-input {
    width: 140px;
    font-family: 'JetBrains Mono', monospace;
  }

  .number-input:focus, .text-input:focus {
    border-color: var(--accent);
    background: rgba(255, 255, 255, 0.1);
  }
</style>
