import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { ServerDetailModal } from '../../../apps/desktop/src/features/registry/ServerDetailModal';
import type { AuthConfig, ServerViewModel } from '../../../apps/desktop/src/types/registry';

function makeServer(overrides: Partial<ServerViewModel> = {}): ServerViewModel {
  return {
    id: 'com.test-server',
    name: 'Test Server',
    description: 'A test MCP server',
    alias: 'test',
    icon: null,
    auth: { type: 'none' },
    transport: {
      type: 'http',
      url: 'https://example.com/mcp',
      headers: {},
      metadata: { inputs: [] },
    },
    categories: ['developer-tools'],
    publisher: null,
    source: { type: 'Registry', url: 'https://registry.mcpmux.com', name: 'McpMux Registry' },
    is_installed: false,
    enabled: false,
    oauth_connected: false,
    input_values: {},
    connection_status: 'disconnected',
    missing_required_inputs: false,
    last_error: null,
    ...overrides,
  };
}

describe('ServerDetailModal', () => {
  const defaultProps = {
    onClose: vi.fn(),
    onInstall: vi.fn(),
    onUninstall: vi.fn(),
  };

  it('should render server name', () => {
    const server = makeServer({ name: 'Cloudflare Workers' });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText('Cloudflare Workers')).toBeInTheDocument();
  });

  it('should render fallback icon when icon is null', () => {
    const server = makeServer({ icon: null });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByTestId('server-icon-fallback')).toHaveTextContent('📦');
  });

  it('should render emoji icon as text', () => {
    const server = makeServer({ icon: '🔐' });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByTestId('server-icon-emoji')).toHaveTextContent('🔐');
  });

  it('should render URL icon as img element', () => {
    const server = makeServer({
      icon: 'https://avatars.githubusercontent.com/u/314135?v=4',
    });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    const img = screen.getByTestId('server-icon-img');
    expect(img.tagName).toBe('IMG');
    expect(img).toHaveAttribute(
      'src',
      'https://avatars.githubusercontent.com/u/314135?v=4'
    );
  });

  it('should render description', () => {
    const server = makeServer({ description: 'Manages KV and R2 buckets' });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText('Manages KV and R2 buckets')).toBeInTheDocument();
  });

  it('should render categories', () => {
    const server = makeServer({ categories: ['cloud', 'developer-tools'] });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText('cloud')).toBeInTheDocument();
    expect(screen.getByText('developer-tools')).toBeInTheDocument();
  });

  it('should render hosting type for remote server', () => {
    const server = makeServer({
      transport: {
        type: 'http',
        url: 'https://example.com/mcp',
        headers: {},
        metadata: { inputs: [] },
      },
    });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText(/Remote Server/)).toBeInTheDocument();
  });

  it('should render Install button for non-installed server', () => {
    const server = makeServer({ is_installed: false });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText('Install')).toBeInTheDocument();
  });

  it('should render Uninstall button for installed server', () => {
    const server = makeServer({ is_installed: true });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText('Uninstall')).toBeInTheDocument();
  });

  it.each<[AuthConfig, string]>([
    [{ type: 'none' }, '✅ No authentication required'],
    [{ type: 'api_key', instructions: null }, '🔑 API Key Required'],
    [{ type: 'optional_api_key', instructions: null }, '🔑 API Key (Optional)'],
    [{ type: 'basic', instructions: null }, '🔑 Username & Password Required'],
    [{ type: 'oauth' }, '🔐 OAuth Authentication'],
  ])('should label auth %j as "%s"', (auth, label) => {
    render(<ServerDetailModal server={makeServer({ auth })} {...defaultProps} />);
    expect(screen.getByText(label)).toBeInTheDocument();
  });

  it('should render basic auth instructions', () => {
    const server = makeServer({
      auth: { type: 'basic', instructions: 'Use your dashboard username and password.' },
    });
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText('Use your dashboard username and password.')).toBeInTheDocument();
  });

  it('should render View JSON button in footer', () => {
    const server = makeServer();
    render(<ServerDetailModal server={server} {...defaultProps} />);
    expect(screen.getByText('View JSON')).toBeInTheDocument();
  });
});
