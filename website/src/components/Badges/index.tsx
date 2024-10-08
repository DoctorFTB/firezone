import Image from "next/image";
import Link from "next/link";

type Size = "xs" | "sm" | "md" | "lg" | "xl";

enum SizeClass {
  XS = "px-1 py-0.5 text-xs rounded-2xl",
  SM = "px-2 py-1 text-sm rounded-2xl",
  MD = "px-2 py-1 text-base rounded-2xl",
  LG = "px-3 py-1.5 text-lg rounded-2xl",
  XL = "px-4 py-2 text-xl rounded-2xl",
}

export function Badge({
  text,
  size,
  bgColor,
  textColor,
}: {
  text: string;
  size: Size;
  bgColor: string;
  textColor: string;
}) {
  const sizeClass = SizeClass[size.toUpperCase() as keyof typeof SizeClass];

  return (
    <div
      className={`place-content-center inline-block ${sizeClass} font-medium bg-${bgColor} text-${textColor}`}
    >
      {text}
    </div>
  );
}

export function RunaCap() {
  return (
    <Link
      href="https://runacap.com/ross-index/q2-2022/"
      target="_blank"
      rel="noopener"
    >
      <Image
        width={260}
        height={56}
        src="https://runacap.com/wp-content/uploads/2022/07/ROSS_badge_black_Q2_2022.svg"
        alt="ROSS Index - Fastest Growing Open-Source Startups in Q2 2022 | Runa Capital"
      />
    </Link>
  );
}
