import clsx from 'clsx';

interface Props {
  className?: string;
}

export function Footer({ className }: Props) {
  return (
    <footer className={clsx('border-t border-[#585858] flex flex-col sm:flex-row  items-center gap-1 text-white/50 py-5', className)}>
      <span>Copyright © 2025 TxPipe</span>
      <span className="hidden sm:block">|</span>
      <span>All Rights Reserved</span>
      {/* <span className="hidden sm:block">|</span>
      <a href="#" className="text-[#83D9FF]/60 underline">Terms and Conditions</a>
      <span className="hidden sm:block">|</span>
      <a href="#" className="text-[#83D9FF]/60 underline">Privacy Policy</a> */}
    </footer>
  );
}
