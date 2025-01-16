import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface Props {
  readme: string | null;
}

export function TabReadme({ readme }: Props) {
  return (
    <Markdown
      className="prose prose-invert max-w-none prose-headings:border-b prose-headings:border-b-white/20 prose-headings:pb-1.5 bg-white/5 p-4 rounded-xl"
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
  );
}
