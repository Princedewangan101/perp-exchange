function Skeleton({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={`animate-pulse rounded bg-zinc-800/60 ${className ?? ""}`}
      {...props}
    />
  );
}

export { Skeleton };
