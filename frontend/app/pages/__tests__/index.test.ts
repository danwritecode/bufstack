import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import IndexPage from '../index.vue'

const mockSayHello = vi.fn()

// useGrpcClient is auto-imported by Nuxt, so stub it as a global
vi.stubGlobal('useGrpcClient', vi.fn(() => Promise.resolve({ sayHello: mockSayHello })))

const stubs = {
  Card: { template: '<div><slot /></div>' },
  CardHeader: { template: '<div><slot /></div>' },
  CardTitle: { template: '<h1><slot /></h1>' },
  CardDescription: { template: '<p><slot /></p>' },
  CardContent: { template: '<div><slot /></div>' },
  CardFooter: { template: '<div><slot /></div>' },
  Button: { template: '<button @click="$emit(\'click\')"><slot /></button>' },
  Badge: { template: '<span><slot /></span>' },
  Alert: { template: '<div><slot /></div>' },
  AlertTitle: { template: '<div><slot /></div>' },
  AlertDescription: { template: '<div><slot /></div>' },
}

beforeEach(() => {
  mockSayHello.mockReset()
})

describe('Index Page', () => {
  it('renders the title and description', () => {
    const wrapper = mount(IndexPage, { global: { stubs } })

    expect(wrapper.text()).toContain('Bufstack')
    expect(wrapper.text()).toContain('Rust gRPC + Nuxt 4 + Protocol Buffers')
  })

  it('calls sayHello and displays the response', async () => {
    mockSayHello.mockResolvedValue({ message: 'Hello World!' })

    const wrapper = mount(IndexPage, { global: { stubs } })

    await wrapper.find('button').trigger('click')

    // Wait for the async gRPC call to resolve and DOM to update
    await vi.waitFor(() => {
      expect(wrapper.text()).toContain('Hello World!')
    })
  })
})
