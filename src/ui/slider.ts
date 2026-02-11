type SliderUpdate = (value: number) => void;

/**
 * Generic throttle: invokes `fn` at most once every `wait` ms.
 * Latest arguments are used when the function is actually called.
 */
export function throttle<T extends unknown[]>(
  fn: (...args: T) => void,
  wait: number
): (...args: T) => void {
  let timeoutId: number | undefined;
  let lastArgs: T | undefined;
  let lastInvoke = 0;

  const invoke = () => {
    if (!lastArgs) return;
    lastInvoke = Date.now();
    fn(...lastArgs);
    lastArgs = undefined;
  };

  return (...args: T) => {
    const now = Date.now();
    const elapsed = now - lastInvoke;
    lastArgs = args;

    if (elapsed >= wait) {
      if (timeoutId !== undefined) {
        window.clearTimeout(timeoutId);
        timeoutId = undefined;
      }
      invoke();
    } else if (timeoutId === undefined) {
      const remaining = wait - elapsed;
      timeoutId = window.setTimeout(() => {
        timeoutId = undefined;
        invoke();
      }, remaining);
    }
  };
}

interface SliderOptions {
  element: HTMLInputElement;
  onUpdate: SliderUpdate;
  min?: number;
  max?: number;
  step?: number;
  throttleMs?: number;
}

export class SliderController {
  private input: HTMLInputElement;
  private valueEl: HTMLElement | null;
  private throttledUpdate: SliderUpdate;
  private min: number;
  private max: number;

  constructor(options: SliderOptions) {
    const {
      element,
      onUpdate,
      min = Number(element.min || 0),
      max = Number(element.max || 100),
      step = Number(element.step || 1),
      throttleMs = 100,
    } = options;

    this.input = element;
    this.valueEl = element
      .closest('.slider')
      ?.querySelector<HTMLElement>('.slider__value') ?? null;

    this.min = min;
    this.max = max;

    this.input.min = String(min);
    this.input.max = String(max);
    this.input.step = String(step);

    this.throttledUpdate = throttle(onUpdate, throttleMs);

    const initialValue = this.currentValue;
    this.updateVisuals(initialValue);
    this.throttledUpdate(initialValue);

    this.input.addEventListener('input', this.handleInput);
  }

  private get currentValue(): number {
    return Number(this.input.value);
  }

  private handleInput = () => {
    const value = this.currentValue;
    this.updateVisuals(value);
    this.throttledUpdate(value);
  };

  private updateVisuals(value: number) {
    if (this.valueEl) {
      this.valueEl.textContent = String(value);
    }

    this.input.setAttribute('aria-valuenow', String(value));

    const percent = ((value - this.min) / (this.max - this.min || 1)) * 100;

    this.input.style.background = `linear-gradient(to right, #3b82f6 ${percent}%, #e5e7eb ${percent}%)`;
  }

  public destroy() {
    this.input.removeEventListener('input', this.handleInput);
  }
}
