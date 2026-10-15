import brandLogo from "../assets/pg-monogram.svg";

export function BrandMark() {
  return (
    <svg viewBox="0 0 100 100" aria-hidden="true">
      <image href={brandLogo} width="100" height="100" />
    </svg>
  );
}

// PG Lab AI wordmark: logo mark + name, sized by the parent's font-size.
// Single source for the brand lockup (home and onboarding).
export function Wordmark() {
  return (
    <span className="wordmark inline-flex items-center gap-[0.4em] text-text [&_svg]:w-[1em] [&_svg]:h-[1em] [&_svg]:shrink-0">
      <BrandMark />
      PG Lab AI
    </span>
  );
}
