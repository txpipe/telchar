// Components
import { CopyIcon } from '~/components/icons/copy';

export function Home() {
  return (
    <main className="relative flex-1 grid place-items-center text-center">
      <div>
        <h1 className="text-5xl font-semibold dark:text-primary">Telchar</h1>
        <p className="mt-6 max-w-[800px] dark:text-[#C5C5C5]">
          A public registry to search and find Plutus blueprint definitions for protocols that were authored by third
          parties. Codegen templates to turn a blueprint into "typings" for popular transaction builder libraries.
        </p>

        <div className="mt-17 p-8 bg-linear-136 dark:from-[#434343]/[0.35] dark:to-transparent rounded-t-xl backdrop-blur-[14.605px]">
          <p className="dark:text-white">
            Run the following command in your terminal, then follow the onscreen instructions.
          </p>

          <p className="mt-8 mx-auto px-8 py-3.5 border dark:border-white/30 rounded-full flex items-center gap-3 font-roboto text-lg dark:text-primary w-fit">
            brew install txpipe/tap/telchar
            <button type="button" onClick={() => navigator.clipboard.writeText('brew install txpipe/tap/telchar')} className="dark:text-white cursor-pointer">
              <CopyIcon />
            </button>
          </p>
        </div>

        <p className="mt-17 dark:text-white/50">
          Need help? Join the support <a href="#" className="underline underline-offset-3">Discord</a> or <a href="#" className="underline underline-offset-3">Read the docs</a>.
        </p>
      </div>
      <img src="/images/logo.svg" className="aspect-[324/383] hidden sm:block sm:w-[180px] md:w-[240px] lg:w-[270px] xl:w-[324px] absolute left-5 -bottom-14 pointer-events-none" />
    </main>
  );
}
