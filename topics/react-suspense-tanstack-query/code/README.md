# code

このトピックは **スニペット中心**。動作確認は既存の React 環境で行う想定。

## v5: `useSuspenseQuery` の最小構成

```tsx
import { Suspense } from "react";
import { QueryClient, QueryClientProvider, useSuspenseQuery } from "@tanstack/react-query";

const queryClient = new QueryClient();

function UserName({ userId }: { userId: string }) {
  const { data } = useSuspenseQuery({
    queryKey: ["user", userId],
    queryFn: async () => {
      const res = await fetch(`/api/users/${userId}`);
      if (!res.ok) throw new Error("failed to fetch user");
      return res.json() as Promise<{ name: string }>;
    },
  });

  return <span>{data.name}</span>;
}

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Suspense fallback={<p>読み込み中...</p>}>
        <UserName userId="1" />
      </Suspense>
    </QueryClientProvider>
  );
}
```

## v4: `useQuery` + `suspense: true`

```tsx
const { data } = useQuery({
  queryKey: ["user", userId],
  queryFn,
  suspense: true,
});
```

## メモ

- Suspense では `isPending` / `isLoading` を UI でほぼ使わない
- エラー表示は ErrorBoundary 側で扱う（ライブラリ/自前どちらでもOK）

