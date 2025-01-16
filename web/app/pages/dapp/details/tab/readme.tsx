import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface Props {
  readme: string | null;
}

export function TabReadme({ readme }: Props) {
  return (
    <Markdown
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
