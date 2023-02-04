# frozen_string_literal: true

describe ::CSVPlusPlus::Graph do
  let(:runtime) { build(:runtime) }

  describe '#depth_first_search' do
    let(:number5) { build(:number, n: 5) }
    let(:ast) { build(:fn_call, name: :multiply, arguments: [number5, number5]) }

    it 'accumulates each value returned by the block' do
      expect(described_class.depth_first_search(ast) { |_n| 1 }).to(eq([1, 1, 1]))
    end

    context 'with a number' do
      let(:ast) { build(:number_one) }

      it 'yields the literal' do
        expect { |block| described_class.depth_first_search(ast, &block) }
          .to(yield_successive_args(ast))
      end
    end

    context 'a function call' do
      it 'yields the function and arguments in order' do
        expect { |block| described_class.depth_first_search(ast, &block) }
          .to(yield_successive_args(ast, number5, number5))
      end
    end
  end

  describe '#dependency_graph' do
    let(:ast) do
      build(:fn_call, name: :multiply, arguments: [build(:variable, id: :bar), build(:variable, id: :foo)])
    end
    let(:variables) do
      {
        bar: build(:number_one),
        foo: build(:fn_call_add),
        dep: ast
      }
    end

    subject { described_class.dependency_graph(variables, runtime) }

    it { is_expected.to(eq({ bar: [], dep: %i[bar foo], foo: [] })) }
  end

  describe '#topological_sort' do
    let(:deps) do
      ::CSVPlusPlus::Graph::DependencyGraph[[['a', %w[b c]], ['b', ['c']], ['c', ['d']], ['d', []]]]
    end

    subject { described_class.topological_sort(deps) }

    it 'orders the keys by their required resolution order' do
      expect(subject).to(eq(%w[d c b a]))
    end

    context "when it can't be resolved" do
      let(:deps) { ::CSVPlusPlus::Graph::DependencyGraph[[['a', ['b']], ['b', ['a']]]] }

      it 'orders the keys by their required resolution order' do
        expect { subject }
          .to(raise_error(::TSort::Cyclic))
      end
    end
  end

  describe '#variable_references' do
    let(:ast) do
      build(
        :fn_call,
        arguments: [
          build(
            :fn_call,
            arguments: [
              build(:fn_call, arguments: [build(:variable_foo), build(:number_one)]),
              build(:number_two)
            ]
          ),
          build(:variable_bar),
          build(:variable, id: 'rownum')
        ]
      )
    end
    let(:include_runtime_variables) { false }

    subject { described_class.variable_references(ast, runtime, include_runtime_variables:) }

    it { is_expected.to(eq(%i[foo bar])) }

    context 'without any references' do
      let(:ast) { build(:number_one) }

      it { is_expected.to(eq([])) }
    end

    context 'with one reference' do
      let(:ast) { build(:fn_call_foo) }

      it { is_expected.to(eq(%i[bar])) }
    end

    context 'with include_runtime_variables: true' do
      let(:include_runtime_variables) { true }

      it { is_expected.to(eq(%i[foo bar rownum])) }
    end
  end
end
