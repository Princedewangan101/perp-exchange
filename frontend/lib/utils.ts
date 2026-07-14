import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const delay = async (ms: number) => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve
    }, ms)
  })
}

export const maskEmail = (email: string) => {
  const [local, domain] = email.split('@');
  if (local.length <= 4) return email;
  return local.slice(0, 2) + '...' + local.slice(-2) + '@' + domain;
}
