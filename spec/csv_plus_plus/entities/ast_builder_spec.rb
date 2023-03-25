# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::ASTBuilder do
  let(:builder) { ::Class.new.extend(described_class) }

  describe '#method_missing' do
    let(:method) { :foo }

    subject { builder.method_missing(method, false) }

    it 'raises for an unknown method' do
      expect { subject }
        .to(raise_error(::NoMethodError))
    end

    context 'with a method_name corresponding to a type' do
      let(:method) { :boolean }

      it 'instantiates instances by their type' do
        expect(subject).to(be_a(::CSVPlusPlus::Entities::Boolean))
      end
    end
  end

  describe '#respond_to?' do
    let(:method) { :foo }
    subject { builder.respond_to?(method) }

    it { is_expected.to(be(false)) }

    context 'with an entity type' do
      let(:method) { :boolean }

      it { is_expected.to(be(true)) }
    end
  end
end
