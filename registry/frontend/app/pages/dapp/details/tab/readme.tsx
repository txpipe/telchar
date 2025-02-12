import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface Props {
  readme: string | null;
}

export function TabReadme({ readme }: Props) {
  return (
    <div className="bg-white/5 p-4 rounded-xl">
      {/* <div className="text-white/[0.24] text-right">
        Source: <span className="underline underline-offset-[3px]">Lorem Ipsum</span>
      </div> */}

      <Markdown
        className="mt-8 prose prose-invert max-w-none prose-headings:border-b prose-headings:border-b-white/20 prose-headings:pb-1.5 prose-pre:whitespace-pre-wrap prose-pre:break-words prose-pre:font-roboto"
        remarkPlugins={[remarkGfm]}
        components={{
          h1: 'h2',
          h2: 'h3',
          h3: 'h4',
          h4: 'h5',
        }}
      >
        {readme}
      </Markdown>
    </div>
  );
}
