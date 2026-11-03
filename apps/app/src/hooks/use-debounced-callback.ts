import { useCallback, useEffect, useRef } from 'react';

export function useDebouncedCallback<T extends unknown[]>(
  callback: (...args: T) => void,
  delay: number,
) {
  const callbackRef = useRef(callback);
  const argsRef = useRef<T | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    callbackRef.current = callback;
  }, [callback]);

  const cancel = useCallback(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }

    argsRef.current = null;
  }, []);

  const flush = useCallback(() => {
    if (!timerRef.current || !argsRef.current) return;

    clearTimeout(timerRef.current);
    timerRef.current = null;

    const args = argsRef.current;
    argsRef.current = null;

    callbackRef.current(...args);
  }, []);

  const run = useCallback(
    (...args: T) => {
      if (timerRef.current) {
        clearTimeout(timerRef.current);
      }

      argsRef.current = args;

      timerRef.current = setTimeout(() => {
        timerRef.current = null;

        const pendingArgs = argsRef.current;
        argsRef.current = null;

        if (pendingArgs) {
          callbackRef.current(...pendingArgs);
        }
      }, delay);
    },
    [delay],
  );

  useEffect(() => cancel, [cancel]);

  return { run, cancel, flush };
}
